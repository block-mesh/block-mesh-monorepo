use crate::database::{get_flag, get_flags};
use crate::error::Error;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "read_flag", skip_all)]
pub async fn read_flag(
    Extension(pool): Extension<PgPool>,
    Path(flag): Path<String>,
) -> Result<Json<Value>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let db_flag = get_flag(&mut transaction, &flag)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(db_flag.value))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlagOut {
    name: String,
    value: Value,
}

#[tracing::instrument(name = "read_flags", skip_all)]
pub async fn read_flags(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<FlagOut>>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let db_flags = get_flags(&mut transaction).await.map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(
        db_flags
            .into_iter()
            .map(|i| FlagOut {
                name: i.name,
                value: i.value,
            })
            .collect(),
    ))
}

pub fn get_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/read-flag/:flag", get(read_flag))
        .route("/read-flags", get(read_flags))
}
