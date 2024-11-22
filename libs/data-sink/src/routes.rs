use crate::data_sink::DataSink;
use crate::database::get_user_and_api_token_by_email;
use crate::errors::Error;
use crate::AppState;
use anyhow::anyhow;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use block_mesh_common::interfaces::server_api::{DigestDataRequest, DigestDataResponse};
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use validator::validate_email;

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let data_sink_db_pool = &state.data_sink_db_pool;
    let mut transaction = create_txn(data_sink_db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "follower_health", skip_all)]
pub async fn follower_health(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let follower_db_pool = &state.follower_db_pool;
    let mut transaction = create_txn(follower_db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

pub async fn digest_data(
    State(state): State<AppState>,
    Json(body): Json<DigestDataRequest>,
) -> Result<Json<DigestDataResponse>, Error> {
    if !validate_email(&body.email) {
        return Err(Error::from(anyhow!("BadEmail")));
    }
    let follower_db_pool = &state.follower_db_pool;
    let mut transaction = create_txn(follower_db_pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &body.email)
        .await?
        .ok_or_else(|| anyhow!("UserNotFound"))?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(transaction).await?;
        return Err(Error::from(anyhow!("ApiTokenNotFound")));
    }
    commit_txn(transaction).await?;
    let data_sink_db_pool = &state.data_sink_db_pool;
    let mut transaction = create_txn(data_sink_db_pool).await?;
    DataSink::create_data_sink(&mut transaction, &user.user_id, body.data).await?;
    commit_txn(transaction).await?;
    Ok(Json(DigestDataResponse { status_code: 200 }))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/follower_health", get(follower_health))
        .route("/version", get(version))
        .route("/digest_data", post(digest_data))
        .with_state(state)
}
