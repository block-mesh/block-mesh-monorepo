use crate::startup::application::AppState;
use crate::ws::connection_manager::CronReportSettings;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CronReportSettings>,
) -> impl IntoResponse {
    if let Some(controller) = state
        .ws_connection_manager
        .broadcaster
        .cron_reports_controller
        .clone()
    {
        controller.update(body).await;
        StatusCode::CREATED.into_response()
    } else {
        tracing::warn!("Cron reports controller is missing");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
