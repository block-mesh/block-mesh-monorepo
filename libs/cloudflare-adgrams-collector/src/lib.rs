use serde_json::Value;
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

pub struct UseCase {
    pub title: String,
    pub icon: String,
    pub href: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |req, ctx| async move {
            let url = req.url()?;
            let id = Uuid::new_v4().to_string();
            let kv = ctx.kv("adgrams")?;
            let s = url.to_string();
            console_log!("s = {:#?}", s);
            let value = Value::from(s.clone());
            kv.put(&id, s)?.execute().await?;
            console_log!("great success");
            Response::from_json(&value)
        })
        .get_async("/all", |req, ctx| async move {
            let kv = ctx.kv("adgrams")?;
            let list = kv.list().execute().await?;
            for key in &list.keys {
                let k = &key.name;
                let value = kv.get(k).text().await?.unwrap_or_default();
                console_log!("k = {} | value = {}", k, value);
            }
            Response::from_json(&Value::Null)
        })
        .run(req, env)
        .await
}
