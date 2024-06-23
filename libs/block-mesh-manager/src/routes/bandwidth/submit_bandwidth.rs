use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::api_token::find_token::find_token;
use crate::database::bandwidth::create_bandwidth_report::create_bandwidth_report;
use crate::database::bandwidth::delete_bandwidth_reports_by_time::delete_bandwidth_reports_by_time;
use crate::database::bandwidth::get_latest_bandwidth_reports::get_latest_bandwidth_reports;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "submit_bandwidth", skip(pool, body), level = "trace", fields(email = body.email), err, ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email != body.email {
        return Err(Error::UserNotFound);
    }

    let bandwidth_report = get_latest_bandwidth_reports(&mut transaction, user.id, 60 * 60)
        .await
        .map_err(Error::from)?;

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
        &pool,
        AggregateName::Download,
        user.id,
    )
    .await?;
    let upload = get_or_create_aggregate_by_user_and_name_no_transaction(
        &pool,
        AggregateName::Upload,
        user.id,
    )
    .await?;

    let latency = get_or_create_aggregate_by_user_and_name_no_transaction(
        &pool,
        AggregateName::Latency,
        user.id,
    )
    .await?;

    update_aggregate(
        &mut transaction,
        download.id.unwrap_or_default(),
        &serde_json::Value::from(
            (bandwidth_report.download_speed.unwrap_or_default()
                + latency.value.as_f64().unwrap_or_default()
                + download_speed)
                / 3.0,
        ),
    )
    .await
    .map_err(Error::from)?;

    update_aggregate(
        &mut transaction,
        upload.id.unwrap_or_default(),
        &serde_json::Value::from(
            (bandwidth_report.upload_speed.unwrap_or_default()
                + upload.value.as_f64().unwrap_or_default()
                + upload_speed)
                / 3.0,
        ),
    )
    .await
    .map_err(Error::from)?;

    update_aggregate(
        &mut transaction,
        latency.id.unwrap_or_default(),
        &serde_json::Value::from(
            (bandwidth_report.latency.unwrap_or_default()
                + latency.value.as_f64().unwrap_or_default()
                + latency_report)
                / 3.0,
        ),
    )
    .await
    .map_err(Error::from)?;

    delete_bandwidth_reports_by_time(&mut transaction, user.id, 60 * 60)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;

    Ok(Json(ReportBandwidthResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
