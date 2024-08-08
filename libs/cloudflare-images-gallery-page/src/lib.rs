use askama::Template;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    images: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImagesResponse {
    pub errors: Vec<String>,
    pub messages: Vec<String>,
    pub result: ImageResults,
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResults {
    pub images: Vec<ImageResult>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResult {
    pub filename: String,
    pub id: String,
    pub meta: Option<HashMap<String, String>>,
    pub requireSignedURLs: bool,
    pub uploaded: String,
    pub variants: Vec<String>,
}

#[event(fetch)]
async fn main(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let api_token = env.secret("IMAGES_API_TOKEN").unwrap().to_string();
    let account_id = env.secret("ACCOUNT_ID").unwrap().to_string();
    let image_delivery = env.secret("DELIVERY_URL").unwrap().to_string();
    let client = Client::new();
    let response = match client
        .get(format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/images/v1",
            account_id
        ))
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return Response::error(e.to_string(), 500),
    };
    let response: serde_json::Value = match response.json().await {
        Ok(j) => j,
        Err(e) => return Response::error(e.to_string(), 500),
    };
    // console_log!("response {:#?}", response);

    let mut images: Vec<String> = Vec::new();
    let result = response.get("result").unwrap().get("images").unwrap();
    result.as_array().unwrap().iter().for_each(|i| {
        images.push(format!(
            "{}/{}/public",
            image_delivery,
            i.get("id").unwrap().to_string().trim_matches('"')
        ))
    });
    // console_log!("images {:#?}", images);

    let response = Home { images }.render().unwrap();
    Response::from_html(response)
}
