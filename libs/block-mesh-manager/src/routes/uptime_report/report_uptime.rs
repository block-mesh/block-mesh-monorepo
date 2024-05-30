use crate::database::api_token::find_token::find_token;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use axum::extract::{ConnectInfo, Query};
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use http::StatusCode;
use sqlx::PgPool;
use std::net::SocketAddr;

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
    tracing::info!("Reporting uptime from {}", addr.ip());

    // let response = Client::post(BLOCK_MESH_IP_WORKER).json()

    create_uptime_report(&mut transaction, user.id).await?;

    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
