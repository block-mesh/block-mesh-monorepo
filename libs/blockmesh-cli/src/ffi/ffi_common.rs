use crate::helpers::http_client;
use crate::login_mode::login_mode;
use chrono::Utc;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Notify;
use tokio::time::sleep;

pub static STATUS: OnceCell<Arc<Mutex<LibState>>> = OnceCell::new();

pub static LOCALHOST_ANDROID: &str = "http://10.0.2.2:8000";

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FFIStatus {
    WAITING,
    RUNNING,
    STOP,
}

#[derive(Clone)]
pub struct LibState {
    pub status: FFIStatus,
    pub notify: Arc<Notify>,
}

impl LibState {
    pub fn new() -> Arc<Mutex<Self>> {
        let notify = Arc::new(Notify::new());
        Arc::new(Mutex::new(Self {
            status: FFIStatus::WAITING,
            notify,
        }))
    }
}

impl Display for FFIStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FFIStatus::WAITING => write!(f, "waiting"),
            FFIStatus::RUNNING => write!(f, "running"),
            FFIStatus::STOP => write!(f, "stop"),
        }
    }
}

impl From<FFIStatus> for i8 {
    fn from(val: FFIStatus) -> i8 {
        match val {
            FFIStatus::WAITING => -1,
            FFIStatus::RUNNING => 1,
            FFIStatus::STOP => 0,
        }
    }
}

pub fn get_status() -> FFIStatus {
    let value = STATUS.get_or_init(LibState::new);
    value.lock().unwrap().status
}

pub fn cancel() {
    let value = STATUS.get_or_init(LibState::new);
    let value = value.lock().unwrap();
    value.notify.notify_waiters();
}

pub fn set_status(status: FFIStatus) {
    let value = STATUS.get_or_init(LibState::new);
    let mut val = value.lock().unwrap();
    val.status = status;
}

pub fn get_notify() -> Arc<Notify> {
    let value = STATUS.get_or_init(LibState::new);
    let val = value.lock().unwrap();
    val.notify.clone()
}

pub fn create_current_thread_runtime() -> Arc<Runtime> {
    let runtime = Arc::new(
        Builder::new_current_thread()
            .thread_name("blockmesh-cli")
            .enable_all()
            .build()
            .unwrap(),
    );
    runtime
}

pub fn debug_stop(_url: &str) {
    let runtime = create_current_thread_runtime();
    set_status(FFIStatus::STOP);
    runtime.block_on(async {
        let notify = get_notify();
        notify.notify_waiters();
    });
    set_status(FFIStatus::WAITING);
}

pub fn run_login_mode(url: &str, email: &str, password: &str) {
    let runtime = create_current_thread_runtime();
    let url = url.to_string();
    let email = email.to_string();
    let password = password.to_string();
    runtime.block_on(async {
        let notify = get_notify();
        set_status(FFIStatus::RUNNING);
        let task = tokio::spawn(async move {
            login_mode(&url, &email, &password, Some("Mobile".to_string())).await
        });
        let cancel_future = tokio::spawn(async move {
            notify.notified().await;
            "Future canceled"
        });
        tokio::select! {
            o = task => {
                set_status(FFIStatus::WAITING);
                eprintln!("Task died {:?}", o)
            },
            o = cancel_future=> {
                set_status(FFIStatus::WAITING);
                eprintln!("Cancel request {:?}", o)
            },
        }
    });
    set_status(FFIStatus::WAITING);
}

pub async fn debug_running(url: &str) {
    set_status(FFIStatus::RUNNING);
    loop {
        let now = Utc::now();
        let _ = http_client()
            .get(format!(
                "{}/health_check?time={}&status={}",
                url,
                now,
                get_status()
            ))
            .send()
            .await;
        sleep(Duration::from_secs(5)).await
    }
}
