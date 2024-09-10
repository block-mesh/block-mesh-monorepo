use crate::startup::application::AppState;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::{debug_handler, Json};
use http::StatusCode;
use std::sync::Arc;

// TODO check user role
#[debug_handler]
pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Some(mut controller) = state
        .ws_connection_manager
        .broadcaster
        .clone()
        .cron_reports_controller
    {
        let stats = controller.stats();
        (
            StatusCode::OK,
            Json(Some(serde_json::to_value(stats).unwrap())),
        )
            .into_response()
    } else {
        tracing::warn!("Cron reports controller is missing");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
