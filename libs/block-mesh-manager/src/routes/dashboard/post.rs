use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use sqlx::PgPool;
use std::sync::Arc;
#[allow(unused_imports)]
use tracing::Level;

use block_mesh_common::interfaces::server_api::DashboardResponse;

use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;
use crate::startup::application::AppState;

#[tracing::instrument(name = "dashboard", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DashboardResponse>, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let data = dashboard_data_extractor(&pool, user.id, state).await?;
    Ok(Json(data))
}
