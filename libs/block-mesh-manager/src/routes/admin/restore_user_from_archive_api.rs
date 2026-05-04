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
    pub team_api_key: Option<String>,
    pub api_key: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RestoreUserFromArchiveResponse {
    pub status: ResponseStatus,
    pub restored: bool,
    pub user_id: Option<Uuid>,
    pub message: Option<String>,
    pub conflict_field: Option<String>,
    pub conflict_value: Option<String>,
    pub conflicting_user_id: Option<Uuid>,
}

#[tracing::instrument(name = "restore_user_from_archive_api", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RestoreUserFromArchiveRequest>,
) -> Result<impl IntoResponse, Error> {
    let admin_key = body
        .team_api_key
        .as_deref()
        .into_iter()
        .chain(body.api_key.as_deref())
        .chain(body.code.as_deref())
        .map(str::trim)
        .find(|key| !key.is_empty())
        .unwrap_or("");
    if admin_key.is_empty() || admin_key != state.team_api_key.trim() {
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
                conflict_field: None,
                conflict_value: None,
                conflicting_user_id: None,
            },
        ),
        RestoreUserFromArchiveResult::AlreadyExists => (
            StatusCode::OK,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Success,
                restored: false,
                user_id: None,
                message: Some("User already exists".to_string()),
                conflict_field: None,
                conflict_value: None,
                conflicting_user_id: None,
            },
        ),
        RestoreUserFromArchiveResult::ArchiveNotFound => (
            StatusCode::NOT_FOUND,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Failure,
                restored: false,
                user_id: None,
                message: Some("Archived user not found".to_string()),
                conflict_field: None,
                conflict_value: None,
                conflicting_user_id: None,
            },
        ),
        RestoreUserFromArchiveResult::Conflict(conflict) => (
            StatusCode::CONFLICT,
            RestoreUserFromArchiveResponse {
                status: ResponseStatus::Failure,
                restored: false,
                user_id: None,
                message: Some(match &conflict.value {
                    Some(value) => format!(
                        "Archived user conflicts with an existing {}: {}",
                        conflict.field.as_str(),
                        value
                    ),
                    None => format!(
                        "Archived user conflicts with an existing {}",
                        conflict.field.as_str()
                    ),
                }),
                conflict_field: Some(conflict.field.as_str().to_string()),
                conflict_value: conflict.value,
                conflicting_user_id: conflict.conflicting_user_id,
            },
        ),
    };

    Ok((response.0, Json(response.1)).into_response())
}
