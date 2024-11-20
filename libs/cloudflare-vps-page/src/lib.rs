use crate::ip::{get_ip_api_is_response, new_ipdata};
use block_mesh_common::interfaces::server_api::VpsResp;
use rustc_hash::FxHashMap;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

mod ip;

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
    if let Ok(Some(v)) = req.headers().get("cf-connecting-ip") {
        headers.insert("cf-connecting-ip".to_string(), v.clone());
    }
    let mut ip_data = new_ipdata(headers);
    get_ip_api_is_response(&mut ip_data).await;
    console_log!("ip_data: {:?}", ip_data);
    let resp: VpsResp = VpsResp {
        ip: req
            .headers()
            .get("cf-connecting-ip")
            .unwrap_or_default()
            .unwrap_or_default(),
        status: 200,
        message: "OK".to_string(),
        asn: ip_data.asn(),
        is_datacenter: ip_data.is_datacenter(),
        is_vps: ip_data.is_vps(),
    };
    Response::from_json(&resp)
}
