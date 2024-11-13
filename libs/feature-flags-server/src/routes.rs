use crate::database::get_flag;
use crate::error::Error;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use block_mesh_common::constants::DeviceType;
use dashmap::DashMap;
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

#[tracing::instrument(name = "read_flag", skip_all, level = "trace")]
pub async fn read_flag(
    Extension(flags_cache): Extension<DashMap<String, Value>>,
    Extension(pool): Extension<PgPool>,
    Path(flag): Path<String>,
) -> Result<Json<Value>, Error> {
    if let Some(f) = flags_cache.get(&flag) {
        return Ok(Json(f.value().clone()));
    }
    let mut transaction = create_txn(&pool).await.map_err(Error::from)?;
    let db_flag = get_flag(&mut transaction, &flag)
        .await
        .map_err(Error::from)?;
    flags_cache.insert(flag, db_flag.value.clone());
    commit_txn(transaction).await.map_err(Error::from)?;
    Ok(Json(db_flag.value))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlagOut {
    name: String,
    value: Value,
}

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/read-flag/:flag", get(read_flag))
        .route(
            &format!("/{}/read-flag/:flag", DeviceType::Extension),
            get(read_flag),
        )
        .route(
            &format!("/{}/read-flag/:flag", DeviceType::Unknown),
            get(read_flag),
        )
        .route(
            &format!("/{}/read-flag/:flag", DeviceType::Cli),
            get(read_flag),
        )
        .route(
            &format!("/{}/read-flag/:flag", DeviceType::AppServer),
            get(read_flag),
        )
        .route(
            &format!("/{}/read-flag/:flag", DeviceType::Worker),
            get(read_flag),
        )
}
