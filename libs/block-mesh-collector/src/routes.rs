use crate::CollectorAppState;
use crate::collector_data::CollectorData;
use crate::errors::Error;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(State(state): State<CollectorAppState>) -> Result<impl IntoResponse, Error> {
    let follower_db_pool = &state.db_pool;
    let mut transaction = create_txn(follower_db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDataCollector {
    pub code: String,
    pub source: String,
}

pub async fn create_data_collector(
    State(state): State<CollectorAppState>,
    Query(params): Query<CreateDataCollector>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, Error> {
    if params.code.is_empty() || params.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        return Err(Error::InternalServer("Bad admin param".to_string()));
    }
    let mut transaction = create_txn(&state.db_pool).await?;
    let _ =
        CollectorData::create_new_collector_data(&mut transaction, &params.source, &body).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}
pub fn get_router(state: CollectorAppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/version", get(version))
        .route("/create_data_collector", post(create_data_collector))
        .with_state(state)
}
