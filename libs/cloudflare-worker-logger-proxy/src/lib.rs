#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use reqwest::Client;
use serde_json::{json, Value};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use uuid::Uuid;
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

fn extract_device_type(log: &Value) -> String {
    if log.is_object() {
        if let Some(device_type) = log.as_object().unwrap().get("device_type") {
            return device_type.to_string();
        }
    }
    "".to_string()
}

fn extract_message(log: &Value) -> String {
    if log.is_object() {
        if let Some(event) = log.as_object().unwrap().get("event") {
            if let Some(message) = event.as_object().unwrap().get("message") {
                return message.to_string();
            }
        }
    }
    "".to_string()
}

#[tracing::instrument(name = "tracing::send_log", ret, err)]
async fn send_log(url: &str, api_key: &str, log: Value) -> anyhow::Result<()> {
    let device_type = extract_device_type(&log);
    let message = extract_message(&log);
    let uuid = Uuid::new_v4();
    let event = json!({
        "data": log,
        "requestId": uuid,
        "namespace": device_type,
        "message": message
    });
    match Client::new()
        .post(url)
        .header("x-api-key", api_key)
        .header("x-service", &device_type)
        .json(&event)
        .send()
        .await
    {
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
    Ok(Response::builder().with_status(200).empty())
    /*
    match req.method() {
        Method::Options => return respond_good(),
        Method::Post => {}
        _ => return Response::error("Only accept POST/OPTIONS requests", 400),
    }
    let url = env.secret("log_url").unwrap().to_string();
    let api_key = env.secret("api_key").unwrap().to_string();
    let body: Value = match req.json().await {
        Ok(json) => json,
        Err(e) => {
            console_error!("Error failed to parse JSON: {}", e);
            return Response::error(e.to_string(), 400);
        }
    };

    if body.is_object() {
        let _ = send_log(&url, &api_key, body).await;
    } else if body.is_array() {
        let array = body.as_array().unwrap();
        for item in array {
            let _ = send_log(&url, &api_key, item.to_owned()).await;
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

     */
}
