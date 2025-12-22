use crate::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use time::Date;
use uuid::Uuid;

pub async fn get_daily_stats_by_day(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    day: &Date,
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
        WHERE day = $1
        AND user_id = $2
        LIMIT 1
        "#,
        day,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(daily_stat)
}
