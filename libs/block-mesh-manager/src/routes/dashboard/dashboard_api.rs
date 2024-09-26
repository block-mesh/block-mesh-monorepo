use axum::extract::State;
use axum::{Extension, Json};
use sqlx::PgPool;
use std::sync::Arc;
#[allow(unused_imports)]
use tracing::Level;

use crate::database::api_token::find_token::find_token;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;
use crate::startup::application::AppState;
use block_mesh_common::interfaces::server_api::{DashboardRequest, DashboardResponse};

#[tracing::instrument(name = "dashboard_api", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DashboardRequest>,
) -> Result<Json<DashboardResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }
    let user_id = user.id;
    transaction.commit().await.map_err(Error::from)?;
    let data = dashboard_data_extractor(&pool, user_id, state).await?;
    Ok(Json(data))
}
