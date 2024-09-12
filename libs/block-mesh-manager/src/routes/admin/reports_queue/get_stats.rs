use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::user::UserRole;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use axum_login::AuthSession;
use http::StatusCode;
use std::sync::Arc;

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
    let _role = user.role;
    if matches!(UserRole::User, _role) {
        return Err(Error::Unauthorized);
    }
    transaction.commit().await?;
    if let Some(mut controller) = state
        .ws_connection_manager
        .broadcaster
        .clone()
        .cron_reports_controller
    {
        let stats = controller.stats();
        Ok((
            StatusCode::OK,
            Json(Some(serde_json::to_value(stats).unwrap())),
        )
            .into_response())
    } else {
        tracing::warn!("Cron reports controller is missing");
        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}
