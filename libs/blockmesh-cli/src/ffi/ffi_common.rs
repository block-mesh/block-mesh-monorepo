use once_cell::sync::OnceCell;
use reqwest::ClientBuilder;
use std::process;
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
    let res = ClientBuilder::new()
        .use_rustls_tls()
        .no_hickory_dns()
        .build()
        .unwrap()
        .get(format!(
            "{}/health_check?RUNNING={}&url={}",
            url,
            get_status(),
            url
        ))
        .send()
        .await;

    let _ = ClientBuilder::new()
        .use_rustls_tls()
        .no_hickory_dns()
        .build()
        .unwrap()
        .get(format!(
            "{}/health_check?url={}pid={}&res={:?}",
            LOCALHOST_2,
            url,
            process::id(),
            res,
        ))
        .send()
        .await;

    // let res = ureq::get(&format!(
    //     "{}/health_check?RUNNING={}&url={}",
    //     url,
    //     get_status(),
    //     url
    // ))
    // .query_pairs(vec![("url", url)])
    // .call();
    // let _ = ureq::get(&format!(
    //     "{}/health_check?RUNNING={}&url={}",
    //     LOCALHOST_2,
    //     get_status(),
    //     url
    // ))
    // .query_pairs(vec![("url", url), ("res", &format!("{:?}", res))])
    // .call();
}
