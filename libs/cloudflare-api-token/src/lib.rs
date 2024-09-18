use askama::Template;
use bcrypt::{hash, verify, DEFAULT_COST};
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetTokenResponse, LoginForm};
use reqwest::Client;
use serde_json::Value;
use std::str::FromStr;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use uuid::Uuid;
use worker::kv::Key;
use worker::*;

const NAMESPACE: &str = "worker-api_tokens";

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

async fn get_key<T>(ctx: &RouteContext<T>, key: &str, namespace: &str) -> Result<Option<String>> {
    console_log!("here x key {}", key);
    let keys = ctx.kv(namespace)?;
    console_log!("here y");
    match keys.get(key).text().await {
        Ok(k) => {
            console_log!("ok k {:#?}", k);
            Ok(k)
        }
        Err(e) => {
            console_log!("Error {:#?}", e);
            Err(Error::from(e))
        }
    }
}

async fn get_key_by_prefix<T>(
    ctx: &RouteContext<T>,
    prefix: &str,
    namespace: &str,
) -> Result<Vec<String>> {
    console_log!("here x prefix {}", prefix);
    let keys = ctx.kv(namespace)?;
    console_log!("here y");
    match keys.list().prefix(prefix.to_string()).execute().await {
        Ok(k) => {
            console_log!("ok k {:#?}", k);
            let key_list: Vec<String> = k.keys.into_iter().map(|k| k.name).collect();
            Ok(key_list)
        }
        Err(e) => {
            console_log!("Error {:#?}", e);
            Err(Error::from(e))
        }
    }
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

pub async fn get_token(email: String, password: String) -> Result<GetTokenResponse> {
    console_log!("get_token");
    let response = Client::new()
        .post("https://api.blockmesh.xyz/api/get_token")
        .header("Content-Type", "application/json")
        .json(&LoginForm {
            email: email.clone(),
            password: password.clone(),
        })
        .send()
        .await
        .unwrap();
    let response = response.json::<GetTokenResponse>().await.unwrap();
    Ok(response)
}

pub async fn check_token(email: String, api_token: String) -> Result<GetTokenResponse> {
    console_log!("check_token");
    let response = Client::new()
        .post("https://api.blockmesh.xyz/api/check_token")
        .header("Content-Type", "application/json")
        .json(&CheckTokenRequest {
            email: email.clone(),
            api_token: Uuid::from_str(&api_token).unwrap(),
        })
        .send()
        .await
        .unwrap();
    let response = response.json::<GetTokenResponse>().await.unwrap();
    Ok(response)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .post_async("/api/get_token", |mut req, ctx| async move {
            let body = req.json::<Value>().await?;
            let email = body
                .get("email")
                .unwrap()
                .to_string()
                .trim_matches('"')
                .to_string();
            let password = body
                .get("password")
                .unwrap()
                .to_string()
                .trim_matches('"')
                .to_string();
            let hashed_password = hash(password.clone(), DEFAULT_COST).unwrap();
            let key = format!("{}:{}", email, hashed_password);
            console_log!("here 3 key = {} get_token", key);
            if let Ok(values) = get_key_by_prefix(&ctx, &email, NAMESPACE).await {
                console_log!("Found from KV {}/{}/{:#?}", email, password, values);
                for v in values {
                    let s: Vec<_> = v.split(':').collect();
                    let kv_email = s[0];
                    let kv_password = s[1];
                    console_log!(
                        "v = {:#?} kv_email = {} kv_password = {}",
                        v,
                        kv_email,
                        kv_password
                    );
                    if verify::<&str>(password.as_ref(), kv_password.as_ref()).unwrap_or(false) {
                        let new_key = format!("{}:{}", kv_email, kv_password);
                        let value = get_key(&ctx, &new_key, NAMESPACE).await?.unwrap();
                        return Response::from_json(&GetTokenResponse {
                            api_token: Some(Uuid::from_str(&value.to_string()).unwrap()),
                            message: None,
                        });
                    }
                }
            }
            // let response = get_token(email, password).await?;
            // Response::from_json(&response)
            console_log!("Not found {}/{}", email, password);
            Response::error("Not found", 500)
        })
        .post_async("/api/check_token", |mut req, ctx| async move {
            console_log!("here 1");
            let body = req.json::<Value>().await?;
            let email = body
                .get("email")
                .unwrap()
                .to_string()
                .trim_matches('"')
                .to_string();
            console_log!("here 2");
            let api_token = body
                .get("api_token")
                .unwrap()
                .to_string()
                .trim_matches('"')
                .to_string();
            let key = format!("{}:{}", email, api_token);
            console_log!("here 3 key = {} check_token", key);
            if let Ok(value) = get_key(&ctx, &key, NAMESPACE).await {
                if let Some(value) = value {
                    console_log!("Found from KV {}/{}/{}", email, api_token, value);
                    return Response::from_json(&GetTokenResponse {
                        api_token: Some(Uuid::from_str(&value.to_string()).unwrap()),
                        message: None,
                    });
                }
            }
            // let response = check_token(email, api_token).await?;
            // Response::from_json(&response)
            console_log!("Not found {}/{}", email, api_token);
            Response::error("Not found", 500)
        })
        .get_async("/", |_, _| async move {
            let response = LoginPage {}.render().unwrap();
            Response::from_html(response)
        })
        .run(req, env)
        .await
}
