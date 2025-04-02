#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
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

#[tracing::instrument(name = "respond_good", skip_all, err)]
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

#[tracing::instrument(name = "get_key", skip_all, err)]
async fn get_key(env: &Env, key: &str, namespace: &str) -> Result<Option<Value>> {
    let keys = env.kv(namespace)?;
    let k = keys.get(key).json::<Value>().await?;
    Ok(k)
}

#[tracing::instrument(name = "write_key", skip_all, err)]
async fn write_key(env: &Env, key: &str, value: &Value, namespace: &str) -> Result<()> {
    let keys = env.kv(namespace)?;
    let value = serde_json::to_string(value)?;
    Ok(keys.put(key, value)?.execute().await?)
}

#[tracing::instrument(name = "get_dynos", skip_all, err)]
pub async fn get_dynos(app_name: &str, token: &str) -> anyhow::Result<HerokuDynoResponse> {
    let url = format!("https://api.heroku.com/apps/{}/dynos", app_name);
    console_log!("get_dynos => url => {}", url);
    let resp = Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.heroku+json; version=3")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let resp = resp.json::<HerokuDynoResponse>().await?;
    Ok(resp)
}

#[tracing::instrument(name = "restart_dyno", skip_all, err)]
pub async fn restart_dyno(app_name: &str, token: &str, dyno: &str) -> anyhow::Result<()> {
    let url = format!("https://api.heroku.com/apps/{}/dynos/{}", app_name, dyno);
    console_log!("restart_dyno => url => {}", url);
    let resp = Client::new()
        .delete(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.heroku+json; version=3")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let resp = resp.json::<Value>().await?;
    Ok(())
}

#[tracing::instrument(name = "restart_all_dynos", skip_all, err)]
pub async fn restart_all_dynos(app_name: &str, token: &str) -> anyhow::Result<()> {
    let url = format!("https://api.heroku.com/apps/{}/dynos", app_name);
    let _ = Client::new()
        .delete(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.heroku+json; version=3")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    Ok(())
}

#[tracing::instrument(name = "restart_all_dynos", skip_all, err)]
pub async fn restart_one_flow(app_name: &str, token: &str, env: &Env) -> anyhow::Result<()> {
    let key = match get_key(env, "last", "restart_round_robin").await? {
        Some(v) => v.as_u64().unwrap_or_default(),
        None => 1,
    };
    let dynos = get_dynos(app_name, token).await?;
    let key = (key + 1) as usize % dynos.0.len();
    let dyno = dynos.0.get(key).ok_or(anyhow!("missing dyno"))?;
    let val = Value::from(key);
    console_log!(
        "{app_name} => val = {:#?} | Restarting dyno = {:#?}",
        val,
        dyno
    );
    let _ = restart_dyno(app_name, token, &dyno.id).await;
    let _ = write_key(env, "last", &val, "restart_round_robin").await;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HerokuDynoResponse(Vec<HerokuDyno>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HerokuDyno {
    pub id: String,
    pub state: String,
    pub r#type: String,
    pub name: String,
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // let router = Router::new();
    // router
    //     .get_async("/", |_req, ctx| async move {
    //         let app_name = ctx.env.var("app_name").unwrap().to_string();
    //         let token = ctx.env.var("token").unwrap().to_string();
    //         let key = match get_key(&ctx, "last", "restart_round_robin").await {
    //             Ok(r) => match r {
    //                 Some(v) => v.as_u64().unwrap_or_default(),
    //                 None => 1,
    //             },
    //             Err(_) => 1,
    //         };
    //         let dynos = get_dynos(&app_name, &token).await.unwrap();
    //         let key = (key + 1) as usize % dynos.0.len();
    //         let dyno = dynos.0.get(key).unwrap();
    //         let val = Value::from(key);
    //         console_log!(
    //             "{app_name} => val = {:#?} | Restarting dyno = {:#?}",
    //             val,
    //             dyno
    //         );
    //         let _ = restart_dyno(&app_name, &token, &dyno.id).await;
    //         let _ = write_key(&ctx, "last", &val, "restart_round_robin").await;
    //         Response::from_json(&dynos)
    //     })
    //     .run(req, env)
    //     .await
    Ok(Response::builder().with_status(200).empty())
}

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let app_name = env.var("app_name").unwrap().to_string();
    let token = env.var("token").unwrap().to_string();
    let _ = restart_one_flow(&app_name, &token, &env).await;
}
