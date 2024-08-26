use crate::domain::daily_stat::DailyStat;
use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};

#[allow(dead_code)]
pub(crate) async fn get_daily_leaderboard(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<DailyStat>> {
    let day = Utc::now().date_naive() - Duration::days(1);
    let daily_stats = sqlx::query_as!(
        DailyStat,
        r#"
           SELECT
           id, user_id, tasks_count, uptime, status, day, created_at
           FROM daily_stats WHERE day = $1
        "#,
        day
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(daily_stats)
}
