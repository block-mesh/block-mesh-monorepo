use block_mesh_manager_database_domain::domain::daily_stat::{DailyStat, DailyStatStatus};
use chrono::{Duration, Utc};
use sqlx::{query_as, Postgres, Transaction};
use uuid::Uuid;

pub async fn get_daily_stats_bonus_not_applied(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<DailyStat>> {
    let now = Utc::now() - Duration::days(1);
    let day = now.date_naive();
    let rows: Vec<DailyStat> = query_as!(
        DailyStat,
        r#"
        SELECT
        id,
        user_id,
        tasks_count,
        status,
        day,
        created_at,
        uptime,
        updated_at,
        ref_bonus,
        ref_bonus_applied
        FROM daily_stats
        WHERE
            user_id = $1
            AND day < $2
            AND status = $3
            AND ref_bonus_applied = false
        ORDER BY day ASC
        LIMIT 1000
        "#,
        user_id,
        day,
        DailyStatStatus::Finalized.to_string()
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(rows)
}
