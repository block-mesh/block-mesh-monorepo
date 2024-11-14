use crate::database::task::count_user_tasks_in_period::count_user_tasks_in_period;
use crate::database::task::create_task::create_task;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::task::TaskMethod;
use database_utils::utils::instrument_wrapper::commit_txn;
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

#[tracing::instrument(name = "create_task_with_token", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = get_user_and_api_token_by_email(&mut transaction, &body.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    let users_tasks_count = count_user_tasks_in_period(&mut transaction, &user.user_id, 60).await?;
    if users_tasks_count > 50 {
        return Err(Error::TooManyTasks);
    }

    let task_id = create_task(
        &mut transaction,
        &user.user_id,
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
