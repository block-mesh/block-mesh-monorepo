use crate::domain::aggregate::AggregateName;
use crate::domain::create_daily_stat::get_or_create_daily_stat;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use crate::domain::notify_worker::notify_worker;
use anyhow::{anyhow, Error};
use axum::extract::Request;
use axum::Json;
use block_mesh_common::interfaces::db_messages::{
    AggregateMessage, AnalyticsMessage, DBMessageTypes, DailyStatMessage, UsersIpMessage,
};
use block_mesh_common::interfaces::server_api::{
    ClientsMetadata, HandlerMode, ReportUptimeRequest, ReportUptimeResponse,
};
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::{HeaderValue, StatusCode};
use http_body_util::BodyExt;
use num_traits::abs;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

pub fn resolve_ip(
    query_ip: &Option<String>,
    header_ip: &Option<&HeaderValue>,
    addr_ip: String,
) -> String {
    if header_ip.is_some() {
        header_ip.unwrap().to_str().unwrap_or_default().to_string()
    } else if query_ip.is_some() {
        query_ip.clone().unwrap().clone()
    } else {
        addr_ip
    }
}

#[tracing::instrument(name = "send_analytics", skip_all)]
async fn send_analytics(
    pool: &PgPool,
    request: Option<Request>,
    user_id: &Uuid,
) -> Result<(), Error> {
    if let Some(request) = request {
        let (_parts, body) = request.into_parts();
        let bytes = body
            .collect()
            .await
            .map_err(|_| anyhow!("Failed Reading Body"))?
            .to_bytes();
        let body_raw = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""));
        if !body_raw.is_empty() {
            if let Ok(metadata) = serde_json::from_str::<ClientsMetadata>(&body_raw) {
                let _ = notify_worker(
                    pool,
                    AnalyticsMessage {
                        msg_type: DBMessageTypes::AnalyticsMessage,
                        user_id: *user_id,
                        depin_aggregator: metadata.depin_aggregator.unwrap_or_default(),
                        version: metadata.version.unwrap_or_default(),
                        device_type: metadata.device_type,
                    },
                )
                .await;
            }
        }
    }
    Ok(())
}

#[tracing::instrument(name = "send_message_to_touch_users_ip", skip_all)]
async fn send_message_to_touch_users_ip(pool: &PgPool, ip: String, user_id: &Uuid) {
    let _ = notify_worker(
        pool,
        UsersIpMessage {
            msg_type: DBMessageTypes::UsersIpMessage,
            id: *user_id,
            ip: ip.clone(),
        },
    )
    .await;
}
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(name = "report_uptime_content", skip_all)]
pub async fn report_uptime_content(
    pool: &PgPool,
    follower_pool: &PgPool,
    ip: String,
    query: ReportUptimeRequest,
    request: Option<Request>,
    mode: HandlerMode,
    polling_interval: f64,
    interval_factor: f64,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let mut follower_transaction = create_txn(follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut follower_transaction, &query.email)
        .await?
        .ok_or_else(|| anyhow!("User Not Found"))?;

    if user.token.as_ref() != &query.api_token {
        return Err(anyhow!("Api Token mismatch"));
    }
    commit_txn(follower_transaction).await?;

    let mut transaction = create_txn(pool).await?;
    let daily_stat = get_or_create_daily_stat(&mut transaction, &user.user_id, None).await?;
    let _ = send_analytics(pool, request, &user.user_id).await;
    send_message_to_touch_users_ip(pool, ip.clone(), &user.user_id).await;
    let uptime = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Uptime,
        &user.user_id,
    )
    .await
    .map_err(Error::from)?;
    commit_txn(transaction).await?;

    let now = Utc::now();
    let diff = now - uptime.updated_at;
    let sec_diff = abs(diff.num_seconds());
    let connected_buffer = env::var("CONNECTED_BUFFER")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap_or(10);

    let uptime_bonus = env::var("UPTIME_BONUS")
        .unwrap_or("1".to_string())
        .parse()
        .unwrap_or(1);

    let (extra, abs) = if (sec_diff
        < connected_buffer
            * ((polling_interval * interval_factor) as i64)
                .checked_div(1_000)
                .unwrap_or(240))
        || mode == HandlerMode::WebSocket
    {
        (
            (uptime_bonus * sec_diff) as f64,
            uptime.value.as_f64().unwrap_or_default() + sec_diff as f64,
        )
    } else {
        (0.0, uptime.value.as_f64().unwrap_or_default())
    };

    if extra > 0.0 {
        let _ = notify_worker(
            pool,
            DailyStatMessage {
                msg_type: DBMessageTypes::DailyStatMessage,
                id: daily_stat.id,
                uptime: extra,
            },
        )
        .await;
    }
    let _ = notify_worker(
        pool,
        AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: uptime.id,
            value: serde_json::Value::from(abs),
        },
    )
    .await;
    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
