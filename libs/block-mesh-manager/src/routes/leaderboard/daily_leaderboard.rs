#[allow(unused_imports)]
use crate::database::leaderboard::get_daily_leaderboard::get_daily_leaderboard;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
#[allow(unused_imports)]
use crate::utils::points::{TASKS_FACTOR, UPTIME_FACTOR};
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{DailyLeaderboard, LeaderBoardUser};
use chrono::{Duration, Utc};
use sqlx::PgPool;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DailyLeaderboard>, Error> {
    let transaction = pool.begin().await.map_err(Error::from)?;
    let day = Utc::now().date_naive() - Duration::days(1);
    let _user = auth.user.ok_or(Error::UserNotFound)?;
    let leaderboard_users: Vec<LeaderBoardUser> = vec![];
    // let leaderboard_users: Vec<LeaderBoardUser> =
    //     get_daily_leaderboard(&mut transaction, UPTIME_FACTOR, TASKS_FACTOR, 5)
    //         .await?
    //         .into_iter()
    //         .map(|i| {
    //             if user.email == i.email {
    //                 LeaderBoardUser {
    //                     email: user.email.clone(),
    //                     points: i.points,
    //                     ips: i.ips,
    //                 }
    //             } else {
    //                 LeaderBoardUser {
    //                     email: "***@***".to_string(),
    //                     points: i.points,
    //                     ips: i.ips,
    //                 }
    //             }
    //         })
    //         .collect();
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(DailyLeaderboard {
        leaderboard_users,
        day,
    }))
}
