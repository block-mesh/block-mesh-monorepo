use serde_json::Value;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

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

#[tracing::instrument(name = "tracing::send_log", ret, err)]
async fn send_log(url: &str, log: Value) -> anyhow::Result<()> {
    match reqwest::Client::new().post(url).json(&log).send().await {
        Ok(r) => {
            console_log!("Successfully sent log to: {} {}", url, r.status());
            Ok(())
        }
        Err(e) => {
            console_error!("Error failed to send log: {}", e);
            return Err(e.into());
        }
    }
}

pub fn respond_good() -> Result<Response> {
    let mut headers = Headers::new();
    headers.append("Access-Control-Allow-Origin", "*")?;
    headers.append("Access-Control-Allow-Methods", "*")?;
    headers.append("Access-Control-Allow-Headers", "*")?;

    Ok(Response::builder()
        .with_headers(headers)
        .with_status(200)
        .empty())
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match req.method() {
        Method::Options => return respond_good(),
        Method::Post => {}
        _ => return Response::error("Only accept POST/OPTIONS requests", 400),
    }
    let url = env.secret("log_url").unwrap().to_string();
    let body: Value = match req.json().await {
        Ok(json) => json,
        Err(e) => {
            console_error!("Error failed to parse JSON: {}", e);
            return Response::error(e.to_string(), 400);
        }
    };

    if body.is_object() {
        let _ = send_log(&url, body).await;
    } else if body.is_array() {
        let array = body.as_array().unwrap();
        for item in array {
            let _ = send_log(&url, item.to_owned()).await;
        }
    }

    let mut headers = Headers::new();
    headers.append("Access-Control-Allow-Origin", "*")?;
    headers.append("Access-Control-Allow-Methods", "*")?;
    headers.append("Access-Control-Allow-Headers", "*")?;

    Ok(Response::builder()
        .with_headers(headers)
        .with_status(200)
        .empty())
}
