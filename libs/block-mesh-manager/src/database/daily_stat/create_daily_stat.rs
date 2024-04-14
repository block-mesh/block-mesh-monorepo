use crate::domain::daily_stat::DailyStatStatus;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Daily Stat", skip(transaction), ret, err)]
pub(crate) async fn create_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let day = now.date_naive();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO daily_stats (id, created_at, user_id, tasks_count, status, day) VALUES ($1, $2, $3, $4, $5, $6)"#,
        id,
        now,
        user_id,
        0,
        DailyStatStatus::OnGoing.to_string(),
        day
    )
        .execute(&mut **transaction)
        .await?;
    Ok(id)
}
