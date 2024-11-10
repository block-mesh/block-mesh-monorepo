use crate::errors::error::Error;
use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::Extension;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::{Method, StatusCode};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    method: Method,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("HEALTH-CHECK:: {:#?} - query = {:#?}", method, query);
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}
