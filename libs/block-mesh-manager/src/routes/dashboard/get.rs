use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate;

#[tracing::instrument(name = "dashboard")]
pub async fn handler() -> impl IntoResponse {
    DashboardTemplate
}
