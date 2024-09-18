use anyhow::Error as AnyhowError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] AnyhowError),
    #[error("User not found")]
    UserNotFound,
    #[error("Api token not found")]
    ApiTokenNotFound,
    #[error("Api token mismatch")]
    ApiTokenMismatch,
    #[error("Password Mismatch")]
    PasswordMismatch,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error occurred: {}", self);
        match self {
            Error::PasswordMismatch => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::UserNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::ApiTokenNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::ApiTokenMismatch => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::Anyhow(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::Sql(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
        }
    }
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        tracing::error!("Error occurred: {}", error);
        match error {
            Error::PasswordMismatch => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ApiTokenMismatch => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UserNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ApiTokenNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Sql(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
