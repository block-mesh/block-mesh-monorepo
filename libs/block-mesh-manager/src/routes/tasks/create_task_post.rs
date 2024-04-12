use crate::database::task::create_task::create_task;
use crate::domain::task::Method;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskForm {
    pub url: String,
    pub method: Method,
    pub headers: Option<JsonValue>,
    pub body: Option<JsonValue>,
}

#[tracing::instrument(name = "create_task_post", skip(auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<CreateTaskForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
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
    Ok(Redirect::to("/dashboard"))
}
