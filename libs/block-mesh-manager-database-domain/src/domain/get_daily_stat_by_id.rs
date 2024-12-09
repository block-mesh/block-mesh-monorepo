use crate::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_daily_stats_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> anyhow::Result<DailyStat> {
    let daily_stat = sqlx::query_as!(
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
        WHERE id = $1
        LIMIT 1
        "#,
        id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(daily_stat)
}
