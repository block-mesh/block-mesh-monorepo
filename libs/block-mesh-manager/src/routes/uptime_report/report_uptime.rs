use crate::database::api_token::find_token::find_token;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use axum::extract::Query;
use axum::{Extension, Json};
use block_mesh_common::interface::{ReportUptimeRequest, ReportUptimeResponse};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "report_uptime", skip(pool, query), err, ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
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
    create_uptime_report(&mut transaction, user.id).await?;

    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
