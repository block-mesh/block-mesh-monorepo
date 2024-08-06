use crate::database::api_token::find_token::find_token;
use crate::database::uptime_report::get_user_latest_uptime::get_user_latest_uptime;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use axum::extract::Query;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetUserUptimeRequest, GetUserUptimeResponse};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "get_user_uptime", skip(pool, query), level = "trace", ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<GetUserUptimeRequest>,
) -> Result<Json<GetUserUptimeResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != query.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }
    let user_latest_uptime = get_user_latest_uptime(&mut transaction, user.id).await?;
    transaction.commit().await.map_err(Error::from)?;
    match user_latest_uptime {
        Some(user_latest_uptime) => {
            return Ok(Json(GetUserUptimeResponse {
                user_id: user.id,
                status_code: u16::from(StatusCode::OK),
                start_time: user_latest_uptime.start_time,
                end_time: user_latest_uptime.end_time,
                duration_seconds: user_latest_uptime.duration_seconds,
            }))
        }
        None => {
            return Ok(Json(GetUserUptimeResponse {
                user_id: user.id,
                status_code: u16::from(StatusCode::NOT_FOUND),
                start_time: None,
                end_time: None,
                duration_seconds: None,
            }))
        }
    }
}
