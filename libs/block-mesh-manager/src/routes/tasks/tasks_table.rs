use crate::database::task::get_tasks_by_user_id::get_tasks_by_user_id;
use crate::domain::task::{Method, Status, Task};
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "tasks_table.html")]
struct TasksTableTemplate {
    tasks: Vec<TaskForTemplate>,
}

#[derive(Serialize, Deserialize)]
struct OptionWrapper<T>(T)
where
    T: Serialize;

impl<T> Display for OptionWrapper<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
struct TaskForTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub method: Method,
    pub headers: OptionWrapper<Value>,
    pub body: OptionWrapper<Value>,
    pub assigned_user_id: OptionWrapper<Uuid>,
    pub status: Status,
    pub response_code: OptionWrapper<i32>,
    pub response_raw: OptionWrapper<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Task> for TaskForTemplate {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            user_id: task.user_id,
            url: task.url,
            method: task.method,
            headers: OptionWrapper(task.headers.unwrap_or(Value::Null)),
            body: OptionWrapper(task.body.unwrap_or(Value::Null)),
            assigned_user_id: OptionWrapper(task.assigned_user_id.unwrap_or(Uuid::nil())),
            status: task.status,
            response_code: OptionWrapper(task.response_code.unwrap_or(0)),
            response_raw: OptionWrapper(task.response_raw.unwrap_or_default()),
            created_at: task.created_at,
        }
    }
}

#[tracing::instrument(name = "tasks_table", skip(auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> impl IntoResponse {
    let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
    let user = auth.user.ok_or(Error::UserNotFound).unwrap();
    let tasks = get_tasks_by_user_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)
        .unwrap();
    transaction.commit().await.map_err(Error::from).unwrap();

    let tasks: Vec<TaskForTemplate> = tasks.into_iter().map(TaskForTemplate::from).collect();

    TasksTableTemplate { tasks }
}
