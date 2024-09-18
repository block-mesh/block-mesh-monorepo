use anyhow::Error as AnyhowError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] AnyhowError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error occurred: {}", self);
        match self {
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
            Error::Sql(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
