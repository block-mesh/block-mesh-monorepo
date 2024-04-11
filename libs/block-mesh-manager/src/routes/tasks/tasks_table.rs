use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "tasks_table.html")]
struct TasksTableTemplate;

#[tracing::instrument(name = "tasks_table")]
pub async fn handler() -> impl IntoResponse {
    TasksTableTemplate
}
