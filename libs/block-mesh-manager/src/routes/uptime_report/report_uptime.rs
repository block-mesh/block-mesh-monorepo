use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::{Query, Request, State};
use axum::Json;
use block_mesh_common::feature_flag_client::{get_flag_value_from_map, FlagValue};
use block_mesh_common::interfaces::server_api::{
    HandlerMode, ReportUptimeRequest, ReportUptimeResponse,
};
use block_mesh_manager_database_domain::domain::report_uptime_content::report_uptime_content;
use chrono::{Duration, Utc};
use http::HeaderMap;
use std::sync::Arc;

#[tracing::instrument(name = "report_uptime", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ReportUptimeRequest>,
    request: Request,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let app_env = get_envar("APP_ENVIRONMENT").await;
    let header_ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    }
    .to_string();
    if state.rate_limit {
        let expiry = get_envar("FILTER_REQUEST_EXPIRY_SECONDS")
            .await
            .parse()
            .unwrap_or(3u64);
        let date = Utc::now() + Duration::milliseconds(expiry as i64);
        let key = format!("report_uptime_{}", header_ip);
        if state.rate_limiter.get(&key).await.is_some() {
            return Ok(Json(ReportUptimeResponse { status_code: 429 }));
        }
        state.rate_limiter.insert(key, Some(date)).await;
        let key = format!("report_uptime_{}", query.api_token);
        if state.rate_limiter.get(&key).await.is_some() {
            return Ok(Json(ReportUptimeResponse { status_code: 429 }));
        }
        state.rate_limiter.insert(key, Some(date)).await;
    }

    let polling_interval = get_flag_value_from_map(
        state.flags.clone(),
        "polling_interval",
        FlagValue::Number(120_000.0),
    )
    .await;
    let polling_interval: f64 =
        <FlagValue as TryInto<f64>>::try_into(polling_interval.to_owned()).unwrap_or_default();
    let interval_factor = get_envar("INTERVAL_FACTOR").await.parse().unwrap_or(10.0);
    report_uptime_content(
        &state.pool,
        &state.follower_pool,
        &state.channel_pool,
        header_ip,
        query,
        Some(request),
        HandlerMode::Http,
        polling_interval,
        interval_factor,
    )
    .await
    .map_err(Error::from)
}
