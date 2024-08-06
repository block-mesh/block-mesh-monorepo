use crate::database::api_token::find_token::find_token;
use crate::database::task::count_user_tasks_in_period::count_user_tasks_in_period;
use crate::database::task::create_task::create_task;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::task::TaskMethod;
use crate::errors::error::Error;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskRequest {
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub api_token: Uuid,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskResponse {
    pub task_id: Uuid,
}

#[tracing::instrument(name = "create_task_with_token", skip(pool, body), fields(email = body.email))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, Error> {
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

    let users_tasks_count = count_user_tasks_in_period(&mut transaction, &user.id, 60).await?;
    if users_tasks_count > 50 {
        return Err(Error::TooManyTasks);
    }

    let task_id = create_task(
        &mut transaction,
        &user.id,
        &body.url,
        &body.method,
        body.headers,
        body.body,
    )
    .await
    .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(CreateTaskResponse { task_id }))
}
