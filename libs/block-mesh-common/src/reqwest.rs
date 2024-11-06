use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub fn http_client() -> Client {
    ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .cookie_store(true)
        .user_agent(format!("curl/8.7.1; {}", env!("CARGO_PKG_VERSION")))
        .no_hickory_dns()
        .use_rustls_tls()
        .build()
        .unwrap_or_default()
}
