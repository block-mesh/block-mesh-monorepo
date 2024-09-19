use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument(name = "ok_get_callback")]
pub async fn ok_handler() -> impl IntoResponse {
    (StatusCode::OK, "Request was successful")
}
