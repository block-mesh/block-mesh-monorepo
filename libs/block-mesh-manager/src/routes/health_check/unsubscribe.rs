use crate::errors::error::Error;
use axum::response::IntoResponse;
use http::StatusCode;

#[tracing::instrument(name = "unsubscribe", skip_all)]
pub async fn unsubscribe() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}
