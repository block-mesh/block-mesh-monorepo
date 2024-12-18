use crate::database::daily_stat::get_daily_stats_bonus_not_applied::get_daily_stats_bonus_not_applied;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use block_mesh_common::interfaces::server_api::AdminReferral;
use block_mesh_manager_database_domain::domain::daily_stat_background_job::DailyStatsBackgroundJob;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use std::env;
use std::sync::Arc;

#[tracing::instrument(name = "admin_referral", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AdminReferral>,
) -> Result<impl IntoResponse, Error> {
    if params.code.is_empty() || params.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        return Ok((StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response());
    }
    let email = params.email.to_lowercase();
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let user = match get_user_opt_by_email(&mut follower_transaction, &email).await? {
        Some(user) => user,
        None => {
            return Ok((StatusCode::NO_CONTENT, "User Not FOUND").into_response());
        }
    };
    let to_do_days = get_daily_stats_bonus_not_applied(&mut follower_transaction, &user.id).await?;
    let to_do_days_len = to_do_days.len();
    commit_txn(follower_transaction).await?;
    let mut write_transaction = create_txn(&state.pool).await?;
    DailyStatsBackgroundJob::create_jobs(&mut write_transaction, to_do_days).await?;
    commit_txn(write_transaction).await?;
    // let messages: Vec<DBMessage> = to_do_days
    //     .into_iter()
    //     .map(|i| {
    //         DBMessage::DailyStatRefBonus(DailyStatRefBonus {
    //             user_id: i.user_id,
    //             daily_stat_id: i.id,
    //             day: i.day,
    //         })
    //     })
    //     .collect();
    // let channel_pool = &state.channel_pool.clone();
    // notify_worker(channel_pool, &messages).await?;
    Ok((StatusCode::OK, format!("to_do_days {}", to_do_days_len)).into_response())
}
