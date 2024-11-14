use crate::domain::aggregate::AggregateName;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use crate::domain::notify_worker::notify_worker;
use anyhow::{anyhow, Error};
use axum::Json;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "submit_bandwidth_content", skip_all)]
pub async fn submit_bandwidth_content(
    pool: &PgPool,
    body: ReportBandwidthRequest,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    let mut transaction = create_txn(pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &body.email)
        .await?
        .ok_or_else(|| anyhow!("User Not Found"))?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(transaction).await?;
        return Err(anyhow!("Api Token Mismatch"));
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
        &user.user_id,
    )
    .await?;
    let upload =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Upload, &user.id)
            .await?;

    let latency = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Latency,
        &user.user_id,
    )
    .await?;

    let _ = notify_worker(
        pool,
        AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: download.id,
            value: serde_json::Value::from(
                (download.value.as_f64().unwrap_or_default() + download_speed) / 2.0,
            ),
        },
    )
    .await;
    let _ = notify_worker(
        pool,
        AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: upload.id,
            value: serde_json::Value::from(
                (upload.value.as_f64().unwrap_or_default() + upload_speed) / 2.0,
            ),
        },
    )
    .await;
    let _ = notify_worker(
        pool,
        AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: latency.id,
            value: serde_json::Value::from(
                (latency.value.as_f64().unwrap_or_default() + latency_report) / 2.0,
            ),
        },
    )
    .await;
    commit_txn(transaction).await?;
    Ok(Json(ReportBandwidthResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
