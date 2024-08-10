use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::api_token::find_token::find_token;
use crate::database::bandwidth::create_bandwidth_report::create_bandwidth_report;
use crate::database::bandwidth::delete_bandwidth_reports_by_time::delete_bandwidth_reports_by_time;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::worker::db_agg::{Table, UpdateBulkMessage};
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "submit_bandwidth", skip(pool, body, state), level = "trace", fields(email = body.email), ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
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

    create_bandwidth_report(&mut transaction, user.id, body)
        .await
        .map_err(Error::from)?;

    let download = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Download,
        user.id,
    )
    .await?;
    let upload = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Upload,
        user.id,
    )
    .await?;

    let latency = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Latency,
        user.id,
    )
    .await?;

    let _ = state
        .tx_sql_agg
        .send(UpdateBulkMessage {
            id: download.id.unwrap_or_default(),
            value: serde_json::Value::from(
                (download.value.as_f64().unwrap_or_default() + download_speed) / 2.0,
            ),
            table: Table::Aggregate,
        })
        .await;
    let _ = state
        .tx_sql_agg
        .send(UpdateBulkMessage {
            id: upload.id.unwrap_or_default(),
            value: serde_json::Value::from(
                (upload.value.as_f64().unwrap_or_default() + upload_speed) / 2.0,
            ),
            table: Table::Aggregate,
        })
        .await;
    let _ = state
        .tx_sql_agg
        .send(UpdateBulkMessage {
            id: latency.id.unwrap_or_default(),
            value: serde_json::Value::from(
                (latency.value.as_f64().unwrap_or_default() + latency_report) / 2.0,
            ),
            table: Table::Aggregate,
        })
        .await;
    transaction.commit().await.map_err(Error::from)?;

    let flag = state
        .flags
        .get("submit_bandwidth_run_background")
        .unwrap_or(&false);

    if *flag {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
            delete_bandwidth_reports_by_time(&mut transaction, user.id, 60 * 60)
                .await
                .map_err(Error::from)
                .unwrap();
            transaction.commit().await.map_err(Error::from).unwrap();
        });
        let _ = state.tx.send(handle).await;
    }

    Ok(Json(ReportBandwidthResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
