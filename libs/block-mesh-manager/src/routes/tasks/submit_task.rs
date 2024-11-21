use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::{Query, Request, State};
use axum::Json;
use block_mesh_common::interfaces::server_api::{
    HandlerMode, SubmitTaskRequest, SubmitTaskResponse,
};
use block_mesh_manager_database_domain::domain::submit_task_content::submit_task_content;
use std::sync::Arc;

#[tracing::instrument(name = "submit_task", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SubmitTaskRequest>,
    request: Request,
) -> Result<Json<SubmitTaskResponse>, Error> {
    submit_task_content(
        &state.pool,
        &state.follower_pool,
        query,
        Some(request),
        HandlerMode::Http,
    )
    .await
    .map_err(Error::from)
}
