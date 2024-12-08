use crate::database::store_log_entry;
use crate::errors::Error;
use crate::LogsDrainAppState;
use anyhow::anyhow;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http_body_util::BodyExt;
use reqwest::StatusCode;
use rsyslog::parser::msg::{HerokuRouter, Raw};
use rsyslog::parser::{Skip, StructuredData};
use rsyslog::Message;
use std::env;
use syslog_rfc5424::parse_message;

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

// https://github.com/vasilakisfil/rsyslog/blob/bca2354f3257ccac25eb9da87b6379a48f5e0373/examples/multiline.rs
type HerokuParser<'a> = Message<'a, Option<&'a str>, Skip, HerokuRouter<'a>>;
type RegularParser<'a> = Message<'a, Option<&'a str>, Vec<StructuredData<'a>>, Raw<'a>>;

pub async fn digest_logs(
    headers: HeaderMap,
    State(state): State<LogsDrainAppState>,
    request: Request,
) -> Result<impl IntoResponse, Error> {
    let (_parts, body) = request.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""));
    if body.contains("host heroku router") || body.contains("host app web") {
        return Ok((StatusCode::OK, "OK"));
    }
    let extra_filter = env::var("EXTRA_FILTER").unwrap_or_default();
    if !extra_filter.is_empty() && body.contains(&extra_filter) {
        return Ok((StatusCode::OK, "OK"));
    }
    let mut transaction = create_txn(&state.logs_drain_pool).await?;
    if let Ok(message) = parse_message(&body) {
        tracing::info!("RFC MESSAGE => {:#?}", message);
    }
    if let Some(content_type) = headers.get("Content-Type") {
        tracing::info!(
            "Content-Type: {}",
            content_type.to_str().unwrap_or_default()
        );
    }
    tracing::info!("METHOD = {:#?}", request.method());
    if let Ok(message) = RegularParser::parse(&body).map_err(|e| e.to_string()) {
        tracing::info!("REGULAR MESSAGE => {:#?}", message);
    } else if let Ok(message) = HerokuParser::parse(&body).map_err(|e| e.to_string()) {
        tracing::info!("HEROKU MESSAGE => {:#?}", message);
    } else {
        tracing::info!("TEXT Body => {:#?}", body);
    }
    store_log_entry(&mut transaction, &body).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}

pub fn get_router(state: LogsDrainAppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/version", get(version))
        .route("/digest_logs", post(digest_logs))
        .with_state(state)
}
