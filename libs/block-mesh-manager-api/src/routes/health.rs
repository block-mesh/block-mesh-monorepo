use crate::error::Error;
use axum::response::IntoResponse;
use axum::Extension;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}
