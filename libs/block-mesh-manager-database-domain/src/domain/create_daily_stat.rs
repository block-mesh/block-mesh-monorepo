use crate::domain::daily_stat::DailyStatStatus;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[tracing::instrument(name = "create_daily_stat", skip_all)]
pub async fn create_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let day = now.date_naive();
    let id = Uuid::new_v4();
    let out = sqlx::query!(
        r#"
           WITH
            extant AS (
                SELECT id FROM daily_stats WHERE user_id = $3 AND day = $6
            ),
            inserted AS (
                INSERT INTO daily_stats (id, created_at, user_id, tasks_count, status, day, uptime)
                SELECT $1, $2, $3, $4, $5, $6, $7
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id
            )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM extant
        "#,
        id,
        now,
        user_id,
        0,
        DailyStatStatus::OnGoing.to_string(),
        day,
        0.0
    )
    .fetch_one(&mut **transaction)
    .await?;
    match out.id {
        Some(id) => Ok(id),
        None => Err(anyhow!("Failed to insert into daily_stats table")),
    }
}
