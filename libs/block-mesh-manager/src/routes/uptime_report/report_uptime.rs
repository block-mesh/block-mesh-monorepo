use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Query};
use axum::{Extension, Json};
use http::StatusCode;
use reqwest::Client;
use sqlx::PgPool;

use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use block_mesh_common::interfaces::ip_data::{IPData, IpDataPostRequest};
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};

use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::api_token::find_token::find_token;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::uptime_report::enrich_uptime_report::enrich_uptime_report;
use crate::database::uptime_report::get_user_uptimes::get_user_uptimes;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;

#[tracing::instrument(name = "report_uptime", skip(pool, query), err, ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
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
                aggregate.id,
                &serde_json::Value::from(sum),
            )
            .await
            .map_err(Error::from)?;
        }
    }
    transaction.commit().await.map_err(Error::from)?;

    tokio::spawn(async move {
        let pool = pool.clone();
        let mut transaction = pool.begin().await.map_err(Error::from).unwrap();
        let ip_data = Client::new()
            .post(BLOCK_MESH_IP_WORKER)
            .json(&IpDataPostRequest {
                ip: match query.ip {
                    None => addr.ip().to_string(),
                    Some(ip) => ip,
                },
            })
            .send()
            .await
            .unwrap()
            .json::<IPData>()
            .await
            .unwrap();
        enrich_uptime_report(&mut transaction, uptime_id, ip_data)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
    });

    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
