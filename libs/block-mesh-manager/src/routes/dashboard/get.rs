use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
struct DashboardTemplate;

#[tracing::instrument(name = "dashboard", skip(_auth))]
pub async fn handler(Extension(_auth): Extension<AuthSession<Backend>>) -> impl IntoResponse {
    DashboardTemplate
}
