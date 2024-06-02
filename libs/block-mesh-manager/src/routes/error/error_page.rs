use crate::errors::error::Error;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Template, Debug, Serialize, Deserialize)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub code: u64,
    pub summary: String,
    pub detailed: String,
    pub go_to: String,
}

#[tracing::instrument(name = "error_page")]
pub async fn handler(error: Query<ErrorTemplate>) -> Result<impl IntoResponse, Error> {
    let error_template = ErrorTemplate {
        code: error.code,
        summary: error.summary.clone(),
        detailed: error.detailed.clone(),
        go_to: error.go_to.clone(),
    };
    Ok(error_template)
}
