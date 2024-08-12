use once_cell::sync::OnceCell;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use tokio::runtime::{Builder, Runtime};

pub static STATUS: OnceCell<Arc<Mutex<i8>>> = OnceCell::new();

pub static CLOUDFLARE: &str = "https://cloudflare-worker-echo-debug.blockmesh.workers.dev";
pub static NGROK: &str = "https://distinct-bison-merely.ngrok-free.app";
pub static LOCALHOST: &str = "http://localhost:8000";
pub static LOCALHOST_2: &str = "http://10.0.2.2:8000";

pub fn get_status() -> i8 {
    let value = STATUS.get_or_init(|| Arc::new(Mutex::new(0)));
    *value.lock().unwrap()
}

pub fn set_status(status: i8) {
    let value = STATUS.get_or_init(|| Arc::new(Mutex::new(0)));
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

pub async fn debug_stop(url: &str) {
    let _ = Client::new()
        .get(format!(
            "{}/health_check?RUNNING={}&url={}",
            url,
            get_status(),
            url
        ))
        .send()
        .await;
}
