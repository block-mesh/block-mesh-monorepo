#![allow(unexpected_cfgs)]

mod types;
mod utils;

use crate::utils::{get_json, get_release};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

pub const LATEST_RELEASE: &str =
    "https://api.github.com/repos/block-mesh/block-mesh-monorepo/releases/latest";

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
    let path = req.path();
    console_log!("path {}", path);
    let latest = match get_release().await {
        Ok(latest) => latest,
        Err(_) => {
            return Response::error("Failed to fetch latest release", 500);
        }
    };
    console_log!("Latest {:#?}", latest);
    if path == "/cli" {
        Response::from_json(&latest.get_cli())
    } else {
        let json_asset = match latest.get_json() {
            Some(asset) => asset,
            None => {
                return Response::error("Failed to find json asset", 500);
            }
        };
        let json = match get_json(&json_asset.browser_download_url).await {
            Ok(json) => json,
            Err(_) => {
                return Response::error("Failed to fetch asset", 500);
            }
        };
        Response::from_json(&json)
    }
}
