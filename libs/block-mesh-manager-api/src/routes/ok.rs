use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn ok_handler() -> impl IntoResponse {
    (StatusCode::OK, "Request was successful")
}
