use axum::http::StatusCode;
use axum::response::IntoResponse;

#[tracing::instrument(name = "ok_handler", skip_all)]
pub async fn ok_handler() -> impl IntoResponse {
    (StatusCode::OK, "Request was successful")
}
