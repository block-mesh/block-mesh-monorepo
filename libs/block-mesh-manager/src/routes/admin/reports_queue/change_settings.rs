use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::ws::connection_manager::CronReportSettings;
use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessageTypes};
use http::StatusCode;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

// TODO check user role
#[debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CronReportSettings>,
) -> Result<impl IntoResponse, Error> {
    let user_id = Uuid::parse_str(
        env::var(BLOCKMESH_SERVER_UUID_ENVAR)
            .context("Could not find SERVER_UUID env var")?
            .as_str(),
    )
    .context("SERVER_UUID evn var contains invalid UUID value")?;

    let _ = state
        .tx_aggregate_agg
        .send_async(AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: user_id,
            value: serde_json::to_value(body.clone())
                .context("Failed to serialize cron reports settings")?,
        })
        .await;
    Ok(StatusCode::CREATED.into_response())
}
