use crate::database::task::get_tasks_by_user_id::get_tasks_by_user_id;
use crate::domain::task::{Task, TaskMethod, TaskStatus};
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::fmt::Display;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "tasks/tasks_table.html")]
struct TasksTableTemplate {
    tasks: Vec<TaskForTemplate>,
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
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
    pub method: TaskMethod,
    pub headers: OptionWrapper<Value>,
    pub body: OptionWrapper<Value>,
    pub assigned_user_id: OptionWrapper<Uuid>,
    pub status: TaskStatus,
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

#[tracing::instrument(name = "tasks_table", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let tasks = get_tasks_by_user_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from).unwrap();

    let tasks: Vec<TaskForTemplate> = tasks.into_iter().map(TaskForTemplate::from).collect();

    Ok(TasksTableTemplate {
        tasks,
        chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
        app_server: BLOCK_MESH_APP_SERVER.to_string(),
        github: BLOCK_MESH_GITHUB.to_string(),
        twitter: BLOCK_MESH_TWITTER.to_string(),
        gitbook: BLOCK_MESH_GITBOOK.to_string(),
        logo: BLOCK_MESH_LOGO.to_string(),
        image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
        support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
        chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
    })
}
