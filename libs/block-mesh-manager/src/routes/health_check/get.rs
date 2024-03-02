use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "base.html")]
struct HealthCheckTemplate;

#[tracing::instrument(name = "Health check")]
pub async fn handler() -> impl IntoResponse {
    HealthCheckTemplate
}
