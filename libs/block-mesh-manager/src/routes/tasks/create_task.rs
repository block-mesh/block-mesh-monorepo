use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "create_task.html")]
struct CreateTaskTemplate;

#[tracing::instrument(name = "create_task")]
pub async fn handler() -> impl IntoResponse {
    CreateTaskTemplate
}
