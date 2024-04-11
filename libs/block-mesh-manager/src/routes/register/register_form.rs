use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[tracing::instrument(name = "register")]
pub async fn handler() -> impl IntoResponse {
    RegisterTemplate
}
