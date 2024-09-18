use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use crate::ws::connection_manager::fetch_latest_cron_settings;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_manager_database_domain::domain::user::UserRole;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = state.pool.begin().await?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &user.id)
        .await?
        .ok_or(Error::UserNotFound)?;
    if !matches!(user.role, UserRole::Admin) {
        return Err(Error::Unauthorized);
    }
    transaction.commit().await?;
    let user_id = Uuid::parse_str(env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    let entry = fetch_latest_cron_settings(&state.pool, &user_id).await?;
    Ok(Json(entry))
}
