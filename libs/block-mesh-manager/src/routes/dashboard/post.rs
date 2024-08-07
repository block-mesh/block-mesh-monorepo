use axum::{Extension, Json};
use axum_login::AuthSession;
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::Level;

use block_mesh_common::interfaces::server_api::DashboardResponse;

use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;

#[tracing::instrument(name = "dashboard post", skip(auth), level = "trace",  err(level = Level::TRACE))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DashboardResponse>, Error> {
    let transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let user_id = user.id;
    transaction.commit().await.map_err(Error::from)?;
    let data = dashboard_data_extractor(&pool, user_id).await?;
    Ok(Json(data))
}
