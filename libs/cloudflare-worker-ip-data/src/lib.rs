use ipapi_is_rust::get_ip_info;
use ipapi_is_rust::response::IpApiIsResponse;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

static IP_HEADERS: [&str; 4] = [
    "cf-connecting-ip",
    "x-real-ip",
    "x-forwarded-for",
    "cf-ipcountry",
];

#[derive(Debug, Serialize, Deserialize)]
struct IPData {
    cf_connecting_ip: Option<String>,
    x_real_ip: Option<String>,
    x_forwarded_for: Option<String>,
    cf_ipcountry: Option<String>,
    ip_api_is_response: Option<IpApiIsResponse>,
}

impl IPData {
    pub fn new(headers: FxHashMap<String, String>) -> Self {
        Self {
            cf_connecting_ip: headers.get("cf-connecting-ip").map(|s| s.to_string()),
            x_real_ip: headers.get("x-real-ip").map(|s| s.to_string()),
            x_forwarded_for: headers.get("x-forwarded-for").map(|s| s.to_string()),
            cf_ipcountry: headers.get("cf-ipcountry").map(|s| s.to_string()),
            ip_api_is_response: None,
        }
    }

    pub async fn get_ip_api_is_response(&mut self) {
        let ip = [
            self.cf_connecting_ip.as_ref(),
            self.x_real_ip.as_ref(),
            self.x_forwarded_for.as_ref(),
        ];
        let ip = ip.iter().find(|ip| ip.is_some());
        if let Some(ip) = ip {
            let ip = ip.as_ref().unwrap();
            let response = get_ip_info(ip).await;
            match response {
                Ok(response) => {
                    self.ip_api_is_response = Some(response);
                }
                Err(e) => {
                    tracing::error!("Error getting IP info: {:?}", e);
                }
            }
        }
    }
}

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut headers: FxHashMap<String, String> = FxHashMap::default();
    req.headers().entries().for_each(|(k, v)| {
        if IP_HEADERS.contains(&k.as_str()) {
            headers.insert(k.clone(), v.clone());
        }
    });
    let mut ip_data = IPData::new(headers);
    ip_data.get_ip_api_is_response().await;
    tracing::info!("IP Data: {:?}", ip_data);
    Response::from_json(&ip_data)
}
