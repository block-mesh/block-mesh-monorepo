use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;

#[derive(Template)]
#[template(path = "tasks_table.html")]
struct TasksTableTemplate;

#[tracing::instrument(name = "tasks_table", skip(_auth))]
pub async fn handler(Extension(_auth): Extension<AuthSession<Backend>>) -> impl IntoResponse {
    TasksTableTemplate
}
