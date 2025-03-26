use crate::errors::Error;
use crate::{get_or_create_id, IdAppState};
use anyhow::{anyhow, Context};
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use block_mesh_common::interfaces::server_api::IdRequest;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use std::env;

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(State(state): State<IdAppState>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&state.db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

pub async fn id(
    headers: HeaderMap,
    State(state): State<IdAppState>,
    Query(query): Query<IdRequest>,
) -> Result<impl IntoResponse, Error> {
    let app_env = env::var("APP_ENVIRONMENT").map_err(|_| Error::Anyhow(anyhow!("Missing env")))?;
    let ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    };
    let mut transaction = create_txn(&state.db_pool).await?;
    let _ = get_or_create_id(
        &mut transaction,
        &query.email,
        &query.api_token,
        &query.fp,
        ip,
    )
    .await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK").into_response())
}
#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
pub fn get_router(state: IdAppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/version", get(version))
        .route("/id", get(id))
        .with_state(state)
}
