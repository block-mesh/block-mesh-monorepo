use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use axum::extract::{ConnectInfo, Query, Request, State};
use axum::Json;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::server_api::{
    HandlerMode, ReportUptimeRequest, ReportUptimeResponse,
};
use block_mesh_manager_database_domain::domain::report_uptime_content::{
    report_uptime_content, resolve_ip,
};
use http::HeaderMap;
use std::net::SocketAddr;
use std::sync::Arc;

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
    let polling_interval = state
        .flags
        .get("polling_interval")
        .unwrap_or(&FlagValue::Number(120_000.0));
    let polling_interval: f64 =
        <FlagValue as TryInto<f64>>::try_into(polling_interval.to_owned()).unwrap_or_default();
    let interval_factor = get_envar("INTERVAL_FACTOR").await.parse().unwrap_or(10.0);
    report_uptime_content(
        &state.pool,
        ip,
        query,
        Some(request),
        HandlerMode::Http,
        polling_interval,
        interval_factor,
    )
    .await
    .map_err(Error::from)
}
