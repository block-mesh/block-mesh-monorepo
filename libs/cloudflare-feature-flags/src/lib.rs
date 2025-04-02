#![allow(unexpected_cfgs)]

use askama::Template;
use serde_json::Value;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::kv::Key;
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
#[template(path = "login.html")]
struct LoginPage {}

#[allow(dead_code)]
async fn write_key<T>(
    ctx: &RouteContext<T>,
    key: &str,
    value: &Value,
    namespace: &str,
) -> Result<()> {
    let keys = ctx.kv(namespace)?;
    let value = serde_json::to_string(value)?;
    Ok(keys.put(key, value)?.execute().await?)
}

async fn get_key<T>(ctx: &RouteContext<T>, key: &str, namespace: &str) -> Result<Option<Value>> {
    let keys = ctx.kv(namespace)?;
    let k = keys.get(key).json::<Value>().await?;
    Ok(k)
}

#[allow(dead_code)]
async fn find_key<T>(ctx: &RouteContext<T>, key: &str, namespace: &str) -> Result<Option<Key>> {
    let keys = ctx.kv(namespace)?;
    let k = keys
        .list()
        .execute()
        .await?
        .keys
        .into_iter()
        .find(|i| i.name == key);
    Ok(k)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/read-flags", |_req, ctx| async move {
            let keys = ctx.kv("feature_flags")?;
            let keys = keys.list().execute().await?.keys;
            Response::from_json(&keys)
        })
        .get_async("/read-flag/:flag", |_req, ctx| async move {
            let flag = ctx.param("flag").unwrap();
            let key = get_key(&ctx, flag, "feature_flags").await?;
            match key {
                None => Response::error(format!("Didn't find flag {}", flag), 500),
                Some(k) => Response::from_json(&k),
            }
        })
        .post_async("/write-flag/:flag", |_req, _ctx| async move {
            // let flag = ctx.param("flag").unwrap();
            // let body = req.json::<Value>().await?;
            // write_key(&ctx, &flag, &body, "feature_flags").await?;
            // let headers = req.headers();
            //
            // let auth = match headers.get("Authorization").unwrap() {
            //     Some(a) => a,
            //     None => return Response::error("Missing Authorization header", 500),
            // };
            // if let None = find_key(&ctx, &auth, "feature_flags_authorization").await? {
            //     return Response::error("Authorization header not found", 500);
            // }
            // let auth_key: Value = match get_key(&ctx, &auth, "feature_flags_authorization").await? {
            //     None => return Response::error("Authorization header not found", 500),
            //     Some(a) => a,
            // };
            // Response::from_json(&body)
            Response::ok("")
        })
        .get_async("/", |_, _| async move {
            let response = LoginPage {}.render().unwrap();
            Response::from_html(response)
        })
        .run(req, env)
        .await
}
