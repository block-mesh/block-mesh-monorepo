use crate::database::task::count_user_tasks_in_period::count_user_tasks_in_period;
use crate::database::task::create_task::create_task;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_manager_database_domain::domain::task::TaskMethod;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskForm {
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
}

#[tracing::instrument(name = "create_task", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<CreateTaskForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let users_tasks_count =
        count_user_tasks_in_period(&mut transaction, &user.id, 60 * 60 * 24).await?;
    if users_tasks_count > 50 {
        return Ok(Error::redirect(
            429,
            "Daily Task Limit Reached",
            "You have reached the daily task limit of 50 tasks",
            "/tasks_table",
        ));
    }
    create_task(
        &mut transaction,
        &user.id,
        &form.url,
        &form.method,
        form.headers,
        form.body,
    )
    .await
    .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Redirect::to("/tasks_table"))
}
