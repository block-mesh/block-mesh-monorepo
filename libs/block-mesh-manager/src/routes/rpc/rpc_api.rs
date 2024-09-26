use axum::{Extension, Json};
use sqlx::PgPool;

use crate::database::task::get_tasks_rpc_results::{get_tasks_rpc_results, RpcResults};
use crate::errors::error::Error;

#[tracing::instrument(name = "rpc_api", skip_all)]
pub async fn handler(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<RpcResults>>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let results = get_tasks_rpc_results(&mut transaction, 600).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(results))
}
