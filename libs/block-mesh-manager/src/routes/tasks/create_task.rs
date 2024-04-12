use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;

#[derive(Template)]
#[template(path = "create_task.html")]
struct CreateTaskTemplate;

#[tracing::instrument(name = "create_task", skip(_auth))]
pub async fn handler(Extension(_auth): Extension<AuthSession<Backend>>) -> impl IntoResponse {
    CreateTaskTemplate
}
