use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[tracing::instrument(name = "login")]
pub async fn handler() -> impl IntoResponse {
    LoginTemplate
}
