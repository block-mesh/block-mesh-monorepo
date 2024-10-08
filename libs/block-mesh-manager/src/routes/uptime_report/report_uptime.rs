use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::database::api_token::find_token::find_token;
use crate::database::daily_stat::create_daily_stat::create_daily_stat;
use crate::database::daily_stat::get_daily_stat_of_user::get_daily_stat_of_user;
use crate::database::daily_stat::increment_uptime::increment_uptime;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use crate::worker::db_cleaner_cron::EnrichIp;
use axum::extract::{ConnectInfo, Query, Request, State};
use axum::Json;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::db_messages::{
    AggregateMessage, AnalyticsMessage, DBMessageTypes, DailyStatMessage, UsersIpMessage,
};
use block_mesh_common::interfaces::server_api::{
    ClientsMetadata, HandlerMode, ReportUptimeRequest, ReportUptimeResponse,
};
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use http::{HeaderMap, HeaderValue, StatusCode};
use http_body_util::BodyExt;
use std::net::SocketAddr;
use std::sync::Arc;
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
    state: Arc<AppState>,
    request: Option<Request>,
    user_id: &Uuid,
) -> Result<(), Error> {
    let tx_analytics_agg = state
        .flags
        .get("tx_analytics_agg")
        .unwrap_or(&FlagValue::Boolean(false));
    let tx_analytics_agg: bool =
        <FlagValue as TryInto<bool>>::try_into(tx_analytics_agg.to_owned()).unwrap_or_default();

    if let Some(request) = request {
        let (_parts, body) = request.into_parts();
        if tx_analytics_agg {
            let bytes = body
                .collect()
                .await
                .map_err(|_| Error::FailedReadingBody)?
                .to_bytes();
            let body_raw = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""));
            if !body_raw.is_empty() {
                if let Ok(metadata) = serde_json::from_str::<ClientsMetadata>(&body_raw) {
                    let _ = state
                        .tx_analytics_agg
                        .send_async(AnalyticsMessage {
                            msg_type: DBMessageTypes::AnalyticsMessage,
                            user_id: *user_id,
                            depin_aggregator: metadata.depin_aggregator.unwrap_or_default(),
                            version: metadata.version.unwrap_or_default(),
                            device_type: metadata.device_type,
                        })
                        .await;
                }
            }
        }
    }
    Ok(())
}

#[tracing::instrument(name = "touch_users_ip", skip_all)]
async fn touch_users_ip(state: Arc<AppState>, ip: String, user_id: &Uuid) {
    let flag = state
        .flags
        .get("touch_users_ip")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    if flag {
        let _ = state
            .tx_users_ip_agg
            .send_async(UsersIpMessage {
                msg_type: DBMessageTypes::UsersIpMessage,
                id: *user_id,
                ip: ip.clone(),
            })
            .await;
    }
}

#[tracing::instrument(name = "send_to_rayon", skip_all)]
async fn send_to_rayon(state: Arc<AppState>, ip: String, user_id: &Uuid) {
    let flag = state
        .flags
        .get("send_cleanup_to_rayon")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    if flag {
        let _ = state
            .cleaner_tx
            .send_async(EnrichIp {
                user_id: *user_id,
                ip: ip.clone(),
            })
            .await;
    }
}

#[tracing::instrument(name = "report_uptime_content", skip_all)]
pub async fn report_uptime_content(
    state: Arc<AppState>,
    ip: String,
    query: ReportUptimeRequest,
    request: Option<Request>,
    mode: HandlerMode,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let pool = state.pool.clone();
    let mut transaction = create_txn(&pool).await?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;

    if user.email.to_ascii_lowercase() != query.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }

    let _ = create_daily_stat(&mut transaction, user.id).await;
    let daily_stat = get_daily_stat_of_user(&mut transaction, user.id).await?;
    let _ = send_analytics(state.clone(), request, &user.id).await;
    touch_users_ip(state.clone(), ip.clone(), &user.id).await;

    let interval = state
        .flags
        .get("polling_interval")
        .unwrap_or(&FlagValue::Number(120_000.0));
    let interval: f64 =
        <FlagValue as TryInto<f64>>::try_into(interval.to_owned()).unwrap_or_default();

    let uptime =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Uptime, &user.id)
            .await
            .map_err(Error::from)?;
    commit_txn(transaction).await?;

    let now = Utc::now();
    let diff = now - uptime.updated_at.unwrap_or(now);
    let interval_factor = get_envar("INTERVAL_FACTOR").await.parse().unwrap_or(10.0);

    let (extra, abs) = if (diff.num_seconds()
        < ((interval * interval_factor) as i64)
            .checked_div(1_000)
            .unwrap_or(240))
        || mode == HandlerMode::WebSocket
    {
        (
            diff.num_seconds() as f64,
            uptime.value.as_f64().unwrap_or_default() + diff.num_seconds() as f64,
        )
    } else {
        (0.0, uptime.value.as_f64().unwrap_or_default())
    };

    if extra > 0.0 {
        let flag = state
            .flags
            .get("report_uptime_daily_stats_via_channel")
            .unwrap_or(&FlagValue::Boolean(false));
        let flag: bool =
            <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();

        if flag {
            let _ = state
                .tx_daily_stat_agg
                .send_async(DailyStatMessage {
                    msg_type: DBMessageTypes::DailyStatMessage,
                    id: daily_stat.id,
                    uptime: extra,
                })
                .await;
        } else {
            let mut transaction = create_txn(&pool).await?;
            let _ = increment_uptime(&mut transaction, &daily_stat.id, extra).await;
            commit_txn(transaction).await?;
        }
    }

    let _ = state
        .tx_aggregate_agg
        .send_async(AggregateMessage {
            msg_type: DBMessageTypes::AggregateMessage,
            id: uptime.id,
            value: serde_json::Value::from(abs),
        })
        .await;

    send_to_rayon(state.clone(), ip.clone(), &user.id).await;

    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}

#[tracing::instrument(name = "report_uptime", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<ReportUptimeRequest>,
    request: Request,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let header_ip = headers.get("cf-connecting-ip");
    let ip = resolve_ip(&query.ip, &header_ip, addr.ip().to_string());
    report_uptime_content(state.clone(), ip, query, Some(request), HandlerMode::Http).await
}
