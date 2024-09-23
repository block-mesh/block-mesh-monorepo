use axum::response::IntoResponse;
use http::StatusCode;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
