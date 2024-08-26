// use crate::database::leaderboard::get_daily_leaderboard::get_daily_leaderboard;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
// use crate::utils::points::raw_points;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::DailyLeaderboard;
// use block_mesh_common::interfaces::server_api::{DailyLeaderboard, LeaderBoardUser};
use chrono::{Duration, Utc};
use sqlx::PgPool;

#[tracing::instrument(name = "daily_leaderboard", skip(_auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(_auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DailyLeaderboard>, Error> {
    // let mut transaction = pool.begin().await.map_err(Error::from)?;
    let day = Utc::now().date_naive() - Duration::days(1);
    // let user = auth.user.ok_or(Error::UserNotFound)?;
    // let mut daily_stats: Vec<LeaderBoardUser> = get_daily_leaderboard(&mut transaction)
    //     .await?
    //     .into_iter()
    //     .map(|i| {
    //         if user.id == i.user_id {
    //             LeaderBoardUser {
    //                 email: user.email.clone(),
    //                 points: raw_points(i.uptime, i.tasks_count),
    //             }
    //         } else {
    //             LeaderBoardUser {
    //                 email: "***@***".to_string(),
    //                 points: raw_points(i.uptime, i.tasks_count),
    //             }
    //         }
    //     })
    //     .collect();
    //
    // daily_stats.sort_by(|a, b| b.cmp(a));
    // let daily_stats = if daily_stats.len() > 5 {
    //     let (daily_stats_top_5, _) = daily_stats.split_at(5);
    //     daily_stats_top_5.to_owned()
    // } else {
    //     daily_stats
    // };

    // let your_rank = daily_stats
    //     .iter()
    //     .position(|i| i.email == user.email)
    //     .unwrap_or_default()
    //     + 1;
    // transaction.commit().await.map_err(Error::from)?;
    Ok(Json(DailyLeaderboard {
        leaderboard_users: vec![],
        day,
    }))
}
