use crate::database::leaderboard::get_daily_leaderboard::get_daily_leaderboard;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::utils::points::{TASKS_FACTOR, UPTIME_FACTOR};
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{DailyLeaderboard, LeaderBoardUser};
use chrono::{Duration, NaiveDate, Utc};
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::OnceCell;

type DailyLeaderBoardCache = Arc<DashMap<NaiveDate, Vec<LeaderBoardUser>>>;

static CACHE: OnceCell<DailyLeaderBoardCache> = OnceCell::const_new();

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DailyLeaderboard>, Error> {
    let day = Utc::now().date_naive() - Duration::days(1);
    let cache = CACHE
        .get_or_init(|| async { Arc::new(DashMap::new()) })
        .await;
    if let Some(entry) = cache.get(&day) {
        let leaderboard_users = entry.value().clone();
        return Ok(Json(DailyLeaderboard {
            leaderboard_users,
            day,
        }));
    }
    let mut transaction = pool.begin().await.map_err(Error::from)?;
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
    transaction.commit().await.map_err(Error::from)?;
    cache.insert(day, leaderboard_users.clone());
    Ok(Json(DailyLeaderboard {
        leaderboard_users,
        day,
    }))
}
