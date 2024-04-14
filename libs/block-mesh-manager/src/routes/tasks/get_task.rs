use crate::database::api_token::find_token::find_token;
use crate::database::task::find_task_by_excluded_user_id_and_status::find_task_by_excluded_user_id_and_status;
use crate::database::task::update_task_assigned::update_task_assigned;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::task::{TaskMethod, TaskStatus};
use crate::errors::error::Error;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskResponse {
    pub id: Uuid,
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
}

#[tracing::instrument(name = "get_task", skip(body))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetTaskRequest>,
) -> Result<Json<GetTaskResponse>, Error> {
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
        find_task_by_excluded_user_id_and_status(&mut transaction, &user.id, TaskStatus::Pending)
            .await?
            .ok_or(Error::TaskNotFound)?;
    update_task_assigned(&mut transaction, task.id, user.id, TaskStatus::Assigned).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetTaskResponse {
        id: task.id,
        url: task.url,
        method: task.method,
        headers: task.headers,
        body: task.body,
    }))
}
