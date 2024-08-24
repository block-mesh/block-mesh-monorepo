#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use reqwest::Client;
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
}

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let app_name = env.var("app_name").unwrap();
    let token = env.var("token").unwrap();
    let url = format!("https://api.heroku.com/apps/{}/dynos", app_name);
    let _ = Client::new()
        .delete(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.heroku+json; version=3")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
}
