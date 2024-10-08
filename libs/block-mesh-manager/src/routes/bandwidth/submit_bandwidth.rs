use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::Json;
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use block_mesh_manager_database_domain::domain::submit_bandwidth_content::submit_bandwidth_content;
use std::sync::Arc;

#[tracing::instrument(name = "submit_bandwidth", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    submit_bandwidth_content(&state.pool, body)
        .await
        .map_err(Error::from)
}
