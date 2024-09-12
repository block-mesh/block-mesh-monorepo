use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::domain::user::UserRole;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use crate::ws::connection_manager::CronReportSettings;
use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use http::StatusCode;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Json(body): Json<CronReportSettings>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = state.pool.begin().await?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &user.id)
        .await?
        .ok_or(Error::UserNotFound)?;
    let _role = user.role;
    if matches!(UserRole::User, _role) {
        return Err(Error::Unauthorized);
    }
    let user_id = Uuid::parse_str(
        env::var(BLOCKMESH_SERVER_UUID_ENVAR)
            .context("Could not find SERVER_UUID env var")?
            .as_str(),
    )
    .context("SERVER_UUID evn var contains invalid UUID value")?;
    let _ = get_or_create_aggregate_by_user_and_name_pool(
        &state.pool,
        AggregateName::CronReports,
        &user_id,
    )
    .await?;
    update_aggregate(
        &mut transaction,
        &user_id,
        &serde_json::to_value(body).context("Failed to parse cron reports settings")?,
    )
    .await?;
    transaction.commit().await?;
    Ok(StatusCode::CREATED.into_response())
}
