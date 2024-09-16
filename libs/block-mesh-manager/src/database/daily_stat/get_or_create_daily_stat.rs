use crate::domain::daily_stat::DailyStat;
use crate::domain::daily_stat::DailyStatStatus;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_or_create_daily_stat", skip(transaction), ret, err)]
pub(crate) async fn get_or_create_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<DailyStat> {
    let now = Utc::now();
    let day = now.date_naive();
    let id = Uuid::new_v4();
    let daily_stat = sqlx::query_as!(
        DailyStat,
        r#"
        INSERT INTO daily_stats
        (id, created_at, user_id, tasks_count, status, day, uptime, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (day, user_id) DO UPDATE SET updated_at = $8
        RETURNING id, created_at, user_id, tasks_count, status, day, uptime, updated_at
        "#,
        id,
        now.clone(),
        user_id,
        0,
        DailyStatStatus::OnGoing.to_string(),
        day,
        0.0,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(daily_stat)
}
