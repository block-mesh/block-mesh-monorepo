use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use crate::ws::connection_manager::fetch_latest_cron_settings;
use crate::ws::cron_reports_controller::CronReportSettings;
use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use block_mesh_manager_database_domain::domain::user::UserRole;
use http::StatusCode;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "change_settings", skip_all)]
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
    if !matches!(user.role, UserRole::Admin) {
        return Err(Error::Unauthorized);
    }
    let server_user_id = Uuid::parse_str(
        env::var(BLOCKMESH_SERVER_UUID_ENVAR)
            .context("Could not find SERVER_UUID env var")?
            .as_str(),
    )
    .context("SERVER_UUID evn var contains invalid UUID value")?;
    let agg = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::CronReports,
        &server_user_id,
    )
    .await?;
    let mut entry = fetch_latest_cron_settings(&state.pool, &server_user_id).await?;
    entry.period = body.period.unwrap_or(entry.period);
    entry.window_size = body.window_size.unwrap_or(entry.window_size);
    entry.messages = body.messages.unwrap_or(entry.messages);
    let value = serde_json::to_value(entry).context("Failed to parse cron aggregate DB entry")?;
    update_aggregate(&mut transaction, &agg.id, &value).await?;
    transaction.commit().await?;
    Ok(StatusCode::CREATED.into_response())
}
