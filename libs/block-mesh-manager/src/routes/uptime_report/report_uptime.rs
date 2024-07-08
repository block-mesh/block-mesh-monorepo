use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, Query, State};
use axum::{Extension, Json};
use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use block_mesh_common::interfaces::ip_data::{IPData, IpDataPostRequest};
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use http::StatusCode;
use reqwest::Client;
use sqlx::PgPool;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::api_token::find_token::find_token;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::uptime_report::delete_uptime_report_by_time::delete_uptime_report_by_time;
use crate::database::uptime_report::enrich_uptime_report::enrich_uptime_report;
use crate::database::uptime_report::get_user_uptimes::get_user_uptimes;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;

#[tracing::instrument(name = "report_uptime", level = "trace", skip(pool, query, state), ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<ReportUptimeRequest>,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email != query.email {
        return Err(Error::UserNotFound);
    }
    let uptime_id = create_uptime_report(&mut transaction, user.id).await?;

    let uptimes = get_user_uptimes(&mut transaction, user.id, 2).await?;
    if uptimes.len() == 2 {
        let diff = uptimes[0].created_at - uptimes[1].created_at;
        if diff.num_seconds() < 60 {
            let aggregate = get_or_create_aggregate_by_user_and_name_no_transaction(
                &pool,
                AggregateName::Uptime,
                user.id,
            )
            .await
            .map_err(Error::from)?;
            let sum = aggregate.value.as_f64().unwrap_or_default() + diff.num_seconds() as f64;
            update_aggregate(
                &mut transaction,
                aggregate.id.0.unwrap_or_default(),
                &serde_json::Value::from(sum),
            )
            .await
            .map_err(Error::from)?;
        }
    }
    delete_uptime_report_by_time(&mut transaction, user.id, 60 * 60)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;

    let handle: JoinHandle<()> = tokio::spawn(async move {
        let _ = enrich_ip(pool.clone(), state.client.clone(), query, addr, uptime_id).await;
    });
    let _ = state.tx.send(handle).await;

    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}

async fn enrich_ip(
    pool: PgPool,
    client: Client,
    query: ReportUptimeRequest,
    addr: SocketAddr,
    uptime_id: Uuid,
) -> anyhow::Result<()> {
    let pool = pool.clone();
    let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
    let ip_data = client
        .post(BLOCK_MESH_IP_WORKER)
        .json(&IpDataPostRequest {
            ip: match &query.ip {
                None => addr.ip().to_string(),
                Some(ip) => ip.to_string(),
            },
        })
        .send()
        .await?
        .json::<IPData>()
        .await?;
    enrich_uptime_report(&mut transaction, uptime_id, ip_data).await?;
    transaction.commit().await?;
}
