use crate::database::{get_flag, get_flags};
use crate::error::Error;
use axum::extract::Path;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

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
        .route("/read-flag/:flag", get(read_flag))
        .route("/read-flags", get(read_flags))
}
