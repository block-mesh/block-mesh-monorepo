use crate::database::api_token::find_token::find_token;
use crate::database::task::find_task_by_task_id_and_status::find_task_by_task_id_and_status;
use crate::database::task::finish_task::finish_task;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskRequest {
    pub email: String,
    pub api_token: Uuid,
    pub task_id: Uuid,
    pub response_code: Option<i32>,
    pub response_raw: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskResponse {
    pub status_code: u16,
}

#[tracing::instrument(name = "submit_task", skip(body))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<SubmitTaskRequest>,
) -> Result<Json<SubmitTaskResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email != body.email {
        return Err(Error::UserNotFound);
    }
    let task =
        find_task_by_task_id_and_status(&mut transaction, &body.task_id, TaskStatus::Assigned)
            .await?
            .ok_or(Error::TaskNotFound)?;
    if task.user_id != user.id {
        return Err(Error::TaskNotFound);
    }

    finish_task(
        &mut transaction,
        body.task_id,
        body.response_code,
        body.response_raw,
        TaskStatus::Completed,
    )
    .await?;

    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(SubmitTaskResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
