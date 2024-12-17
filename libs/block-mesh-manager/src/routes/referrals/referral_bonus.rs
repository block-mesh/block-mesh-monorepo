use crate::database::daily_stat::get_daily_stats_bonus_not_applied::get_daily_stats_bonus_not_applied;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_manager_database_domain::domain::daily_stat_background_job::DailyStatsBackgroundJob;
use chrono::{Duration, Utc};
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;

static RATE_USER: OnceCell<DashSetWithExpiry<Uuid>> = OnceCell::const_new();

#[tracing::instrument(name = "referral_bonus", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let cache = RATE_USER
        .get_or_init(|| async { DashSetWithExpiry::new() })
        .await;
    if cache.get(&user.id).is_some() {
        return Ok((StatusCode::TOO_MANY_REQUESTS, "Rate limited").into_response());
    }
    // let date = Utc::now() + Duration::milliseconds(600_000);
    let date = Utc::now() + Duration::milliseconds(1_000);
    cache.insert(user.id, Some(date));
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
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
