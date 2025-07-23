#![allow(unexpected_cfgs)]

use askama::Template;
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

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    pub chrome_extension_link: String,
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let response = Home {
        chrome_extension_link: "https://extension-releases.perceptrons.xyz/pcn-latest.zip"
            .to_string(),
    }
    .render()
    .unwrap();
    Response::from_html(response)
}
