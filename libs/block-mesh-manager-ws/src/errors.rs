use axum::response::{IntoResponse, Response};
use http::StatusCode;
use std::fmt::{Display, Formatter};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    FailedReadingBody(String),
    ApiTokenNotFound(String),
    UserNotFound(String),
    TaskNotFound(String),
    TaskAssignedToAnotherUser(String),
    InternalServer(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
    }
}
