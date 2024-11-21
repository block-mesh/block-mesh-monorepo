use crate::constants::DeviceType;
use reqwest::{Client, ClientBuilder};
#[allow(unused_imports)]
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
pub fn http_client(device_type: DeviceType) -> Client {
    ClientBuilder::new()
        .user_agent(format!(
            "curl/8.7.1; {}; {}",
            device_type,
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn http_client(device_type: DeviceType) -> Client {
    ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .cookie_store(true)
        .user_agent(format!(
            "curl/8.7.1; {}; {}",
            device_type,
            env!("CARGO_PKG_VERSION")
        ))
        .no_hickory_dns()
        .use_rustls_tls()
        .build()
        .unwrap_or_default()
}
