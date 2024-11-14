use crate::database::uptime_report::get_user_latest_uptime::get_user_latest_uptime;
use crate::errors::error::Error;
use axum::extract::Query;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetUserUptimeRequest, GetUserUptimeResponse};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "get_user_uptime", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<GetUserUptimeRequest>,
) -> Result<Json<GetUserUptimeResponse>, Error> {
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &query.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.token.as_ref() != &query.api_token {
        commit_txn(transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    let user_latest_uptime = get_user_latest_uptime(&mut transaction, user.id).await?;
    transaction.commit().await.map_err(Error::from)?;
    match user_latest_uptime {
        Some(user_latest_uptime) => Ok(Json(GetUserUptimeResponse {
            user_id: user.user_id,
            status_code: u16::from(StatusCode::OK),
            start_time: user_latest_uptime.start_time,
            end_time: user_latest_uptime.end_time,
            duration_seconds: user_latest_uptime.duration_seconds,
        })),
        None => Ok(Json(GetUserUptimeResponse {
            user_id: user.user_id,
            status_code: u16::from(StatusCode::NOT_FOUND),
            start_time: None,
            end_time: None,
            duration_seconds: None,
        })),
    }
}
