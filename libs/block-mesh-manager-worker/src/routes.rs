use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use reqwest::StatusCode;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub fn get_router() -> Router {
    Router::new().route("/health", get(health))
}
