#![allow(unexpected_cfgs)]
use rsyslog::parser::msg::Raw;
use rsyslog::parser::StructuredData;
use rsyslog::{
    parser::{msg::HerokuRouter, Skip},
    Message,
};
use serde_json::Value;
use syslog_rfc5424::parse_message;
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

// https://github.com/vasilakisfil/rsyslog/blob/bca2354f3257ccac25eb9da87b6379a48f5e0373/examples/multiline.rs
type HerokuParser<'a> = Message<'a, Option<&'a str>, Skip, HerokuRouter<'a>>;
type RegularParser<'a> = Message<'a, Option<&'a str>, Vec<StructuredData<'a>>, Raw<'a>>;

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let headers = req.headers();
    if let Ok(Some(content_type)) = headers.get("Content-Type") {
        console_log!("Content-Type: {}", content_type);
    }
    console_log!("METHOD = {:#?}", req.method());
    if let Ok(query) = req.query::<Value>() {
        console_log!("QUERY = {:#?}", query);
    }
    if let Ok(body) = req.text().await {
        if let Ok(message) = parse_message(&body) {
            console_log!("RFC MESSAGE => {:#?}", message);
        }
        if let Ok(message) = RegularParser::parse(&body).map_err(|e| e.to_string()) {
            console_log!("REGULAR MESSAGE => {:#?}", message);
        } else if let Ok(message) = HerokuParser::parse(&body).map_err(|e| e.to_string()) {
            console_log!("HEROKU MESSAGE => {:#?}", message);
        } else {
            console_log!("TEXT Body => {:#?}", body);
        }
    }
    // if let Ok(body) = req.bytes().await {
    //     console_log!("BYTES => {:#?}", body);
    // }
    // if let Ok(body) = req.json::<Value>().await {
    //     console_log!("JSON Body => {:#?}", body)
    // }
    // if let Ok(body) = req.form_data().await {
    //     console_log!("FORM => {:#?}", body);
    // }
    Ok(Response::builder()
        // .with_headers(headers)
        .with_status(200)
        .empty())
}
