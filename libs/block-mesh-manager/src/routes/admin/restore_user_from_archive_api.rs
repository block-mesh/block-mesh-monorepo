use crate::database::user::restore_user_from_archive::{
    restore_user_from_archive, RestoreUserFromArchiveResult,
};
use crate::errors::error::Error;
use crate::routes::basic_response::ResponseStatus;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RestoreUserFromArchiveRequest {
    pub email: String,
    pub team_api_key: String,
}

#[derive(Debug, Serialize)]
pub struct RestoreUserFromArchiveResponse {
    pub status: ResponseStatus,
    pub restored: bool,
    pub user_id: Option<Uuid>,
    pub message: Option<String>,
}

#[tracing::instrument(name = "restore_user_from_archive_api", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RestoreUserFromArchiveRequest>,
) -> Result<impl IntoResponse, Error> {
    if body.team_api_key.is_empty() || body.team_api_key != state.team_api_key.as_str() {
        return Err(Error::Unauthorized);
    }

    let email = body.email.trim().to_ascii_lowercase();
    if email.is_empty() {
        return Err(Error::BadRequest("email is required".to_string()));
    }

    let mut transaction = create_txn(&state.pool).await?;
    let result = restore_user_from_archive(&mut transaction, &email).await?;
    commit_txn(transaction).await?;

    let response = match result {
        RestoreUserFromArchiveResult::Restored(user_id) => (
            StatusCode::CREATED,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Success,
                restored: true,
                user_id: Some(user_id),
                message: Some("User restored from archive".to_string()),
            },
        ),
        RestoreUserFromArchiveResult::AlreadyExists => (
            StatusCode::OK,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Success,
                restored: false,
                user_id: None,
                message: Some("User already exists".to_string()),
            },
        ),
        RestoreUserFromArchiveResult::ArchiveNotFound => (
            StatusCode::NOT_FOUND,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Failure,
                restored: false,
                user_id: None,
                message: Some("Archived user not found".to_string()),
            },
        ),
        RestoreUserFromArchiveResult::Conflict => (
            StatusCode::CONFLICT,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Failure,
                restored: false,
                user_id: None,
                message: Some("Archived user conflicts with an existing unique value".to_string()),
            },
        ),
    };

    Ok((response.0, Json(response.1)).into_response())
}
