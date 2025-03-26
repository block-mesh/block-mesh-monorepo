use anyhow::Error as AnyhowError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] AnyhowError),
}

impl Error {
    pub fn redirect(code: u64, summary: &str, detailed: &str, go_to: &str) -> Redirect {
        let mut url = Url::parse("tmp://error").unwrap();
        url.query_pairs_mut()
            .append_pair("code", &format!("{}", code))
            .append_pair("summary", summary)
            .append_pair("go_to", go_to)
            .append_pair("detailed", detailed);
        let url = url.to_string();
        let url = url.split("tmp:/").last().unwrap();
        Redirect::to(url)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error occurred: {}", self);
        match self {
            Error::Sql(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into_response()
            }
            Error::Anyhow(_) => {
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
