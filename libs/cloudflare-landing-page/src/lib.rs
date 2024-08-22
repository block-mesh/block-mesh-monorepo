use askama::Template;
use serde_json::Value;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT, BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};

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
#[allow(dead_code)]
#[template(path = "home.html")]
struct Home {
    pub counter: u64,
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    // pub image: String,
    pub support: String,
    pub chat: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |_req, ctx| async move {
            let counter = ctx.kv("ab_testing")?.get("counter").json::<Value>().await?;
            let counter = if counter.is_none() {
                let value = serde_json::to_string(&Value::from(0))?;
                ctx.kv("ab_testing")?
                    .put("counter", value)?
                    .execute()
                    .await?;
                0
            } else {
                let c = counter.unwrap().as_u64().unwrap();
                let value = serde_json::to_string(&Value::from(c + 1))?;
                ctx.kv("ab_testing")?
                    .put("counter", value)?
                    .execute()
                    .await?;
                c + 1
            };

            console_log!("counter = {:?}", counter);
            let response = Home {
                counter,
                chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
                app_server: BLOCK_MESH_APP_SERVER.to_string(),
                github: BLOCK_MESH_GITHUB.to_string(),
                twitter: BLOCK_MESH_TWITTER.to_string(),
                gitbook: BLOCK_MESH_GITBOOK.to_string(),
                logo: BLOCK_MESH_LOGO.to_string(),
                // image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
                support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
                chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
            }
            .render()
            .unwrap();
            Response::from_html(response)
        })
        .run(req, env)
        .await
}
