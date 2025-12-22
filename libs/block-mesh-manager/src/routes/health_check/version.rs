use axum::response::IntoResponse;
use http::StatusCode;

#[tracing::instrument(name = "version", skip_all)]
pub async fn handler() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
