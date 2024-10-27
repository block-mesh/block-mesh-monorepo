use crate::errors::Error;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/version", get(version))
}
