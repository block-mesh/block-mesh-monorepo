use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use std::sync::Arc;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    let pool = state.pool.clone();
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}
