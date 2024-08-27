use crate::helpers::http_client;
use chrono::Utc;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;

pub static STATUS: OnceCell<Arc<Mutex<FFIStatus>>> = OnceCell::new();

pub static CLOUDFLARE: &str = "https://cloudflare-worker-echo-debug.blockmesh.workers.dev";
pub static NGROK: &str = "https://distinct-bison-merely.ngrok-free.app";
pub static LOCALHOST: &str = "http://localhost:8000";
pub static LOCALHOST_2: &str = "http://10.0.2.2:8000";

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FFIStatus {
    WAITING,
    RUNNING,
    STOP,
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

pub fn get_status() -> FFIStatus {
    let value = STATUS.get_or_init(|| Arc::new(Mutex::new(FFIStatus::WAITING)));
    *value.lock().unwrap()
}

pub fn set_status(status: FFIStatus) {
    let value = STATUS.get_or_init(|| Arc::new(Mutex::new(FFIStatus::WAITING)));
    let mut val = value.lock().unwrap();
    *val = status;
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

pub fn debug_stop(url: &str) {
    let runtime = create_current_thread_runtime();
    set_status(FFIStatus::STOP);
    runtime.block_on(async {
        let _ = http_client()
            .get(format!(
                "{}/health_check?status={}&url={}",
                url,
                get_status(),
                url
            ))
            .send()
            .await;
    });
    set_status(FFIStatus::WAITING);
}

pub fn debug_running(url: &str) {
    let runtime = create_current_thread_runtime();
    runtime.block_on(async {
        set_status(FFIStatus::RUNNING);
        loop {
            if get_status() != FFIStatus::RUNNING {
                break;
            }
            let now = Utc::now();
            let _ = Client::new()
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
        // let _ = login_mode(url, email, password).await;
    });
    set_status(FFIStatus::WAITING);
}
