use block_mesh_manager_database_domain::domain::daily_stat::DailyStat;
use block_mesh_manager_database_domain::domain::daily_stat::DailyStatStatus;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn get_or_create_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<DailyStat> {
    let now = OffsetDateTime::now_utc();
    let day = now.date();
    let id = Uuid::new_v4();
    let daily_stat = sqlx::query_as!(
        DailyStat,
        r#"
        INSERT INTO daily_stats
        (id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (status, day, user_id) DO UPDATE SET updated_at = $8
        RETURNING id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied
        "#,
        id,
        now,
        user_id,
        0,
        DailyStatStatus::OnGoing.to_string(),
        day,
        0.0,
        now,
        0.0,
        false
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(daily_stat)
}
