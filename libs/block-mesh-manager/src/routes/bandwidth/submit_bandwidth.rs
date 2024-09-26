use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::api_token::find_token::find_token;
use crate::database::bandwidth::delete_bandwidth_reports_by_time::delete_bandwidth_reports_by_time;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::Json;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[allow(dead_code)]
async fn submit_bandwidth_run_background(state: Arc<AppState>, pool: PgPool, user_id: &Uuid) {
    let user_id = *user_id;
    let flag = state
        .flags
        .get("submit_bandwidth_run_background")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    if flag {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
            delete_bandwidth_reports_by_time(&mut transaction, user_id, 60 * 60)
                .await
                .map_err(Error::from)
                .unwrap();
            transaction.commit().await.map_err(Error::from).unwrap();
        });
        let _ = state.tx.send_async(handle).await;
    }
}

#[tracing::instrument(name = "submit_bandwidth_content", skip_all)]
pub async fn submit_bandwidth_content(
    state: Arc<AppState>,
    body: ReportBandwidthRequest,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    let pool = state.pool.clone();
    let mut transaction = pool.begin().await?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }

    let download_speed = serde_json::Value::from(body.download_speed)
        .as_f64()
        .unwrap_or_default();
    let upload_speed = serde_json::Value::from(body.upload_speed)
        .as_f64()
        .unwrap_or_default();
    let latency_report = serde_json::Value::from(body.latency)
        .as_f64()
        .unwrap_or_default();

    let download = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Download,
        &user.id,
    )
    .await?;
    let upload =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Upload, &user.id)
            .await?;

    let latency = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Latency,
        &user.id,
    )
    .await?;
    let flag = state
        .flags
        .get("submit_bandwidth_via_channel")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();

    if flag {
        let _ = state
            .tx_aggregate_agg
            .send_async(AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: download.id,
                value: serde_json::Value::from(
                    (download.value.as_f64().unwrap_or_default() + download_speed) / 2.0,
                ),
            })
            .await;
        let _ = state
            .tx_aggregate_agg
            .send_async(AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: upload.id,
                value: serde_json::Value::from(
                    (upload.value.as_f64().unwrap_or_default() + upload_speed) / 2.0,
                ),
            })
            .await;
        let _ = state
            .tx_aggregate_agg
            .send_async(AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: latency.id,
                value: serde_json::Value::from(
                    (latency.value.as_f64().unwrap_or_default() + latency_report) / 2.0,
                ),
            })
            .await;
    } else {
        let _ = update_aggregate(
            &mut transaction,
            &latency.id,
            &serde_json::Value::from(
                (latency.value.as_f64().unwrap_or_default() + latency_report) / 2.0,
            ),
        )
        .await;
        let _ = update_aggregate(
            &mut transaction,
            &upload.id,
            &serde_json::Value::from(
                (upload.value.as_f64().unwrap_or_default() + upload_speed) / 2.0,
            ),
        )
        .await;
        let _ = update_aggregate(
            &mut transaction,
            &download.id,
            &serde_json::Value::from(
                (download.value.as_f64().unwrap_or_default() + download_speed) / 2.0,
            ),
        )
        .await;
    }
    transaction.commit().await?;

    Ok(Json(ReportBandwidthResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    submit_bandwidth_content(state, body).await
}
