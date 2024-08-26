use crate::database::leaderboard::get_daily_leaderboard::get_daily_leaderboard;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::utils::points::{TASKS_FACTOR, UPTIME_FACTOR};
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{DailyLeaderboard, LeaderBoardUser};
use chrono::{Duration, Utc};
use sqlx::PgPool;

#[tracing::instrument(name = "daily_leaderboard", skip(auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DailyLeaderboard>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let day = Utc::now().date_naive() - Duration::days(1);
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let leaderboard_users: Vec<LeaderBoardUser> =
        get_daily_leaderboard(&mut transaction, UPTIME_FACTOR, TASKS_FACTOR, 5)
            .await?
            .into_iter()
            .map(|i| {
                if user.email == i.email {
                    LeaderBoardUser {
                        email: user.email.clone(),
                        points: i.points,
                    }
                } else {
                    LeaderBoardUser {
                        email: "***@***".to_string(),
                        points: i.points,
                    }
                }
            })
            .collect();

    // let your_rank = daily_stats
    //     .iter()
    //     .position(|i| i.email == user.email)
    //     .unwrap_or_default()
    //     + 1;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(DailyLeaderboard {
        leaderboard_users,
        day,
    }))
}
