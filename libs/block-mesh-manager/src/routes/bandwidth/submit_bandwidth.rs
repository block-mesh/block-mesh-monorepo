use crate::errors::error::Error;
use crate::middlewares::rate_limit::filter_request;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::State;
use axum::Json;
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use block_mesh_manager_database_domain::domain::submit_bandwidth_content::submit_bandwidth_content;
use http::HeaderMap;
use std::sync::Arc;

#[tracing::instrument(name = "submit_bandwidth", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ReportBandwidthRequest>,
) -> Result<Json<ReportBandwidthResponse>, Error> {
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
    let mut redis = state.redis.clone();
    if state.submit_bandwidth_limit {
        let filter =
            filter_request(&mut redis, &body.api_token, &header_ip, "submit_bandwidth").await;
        if filter.is_err() || !filter? {
            return Ok(Json(ReportBandwidthResponse { status_code: 429 }));
        }
    }
    submit_bandwidth_content(&state.pool, &state.follower_pool, &state.channel_pool, body)
        .await
        .map_err(Error::from)
}
