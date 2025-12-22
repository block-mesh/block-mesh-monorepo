use crate::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

#[tracing::instrument(name = "get_daily_stat_of_user", skip_all)]
pub async fn get_daily_stat_of_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<DailyStat> {
    let now = OffsetDateTime::now_utc();
    let day = now.date();
    let dail_stat = sqlx::query_as!(
        DailyStat,
        r#"
        SELECT
        id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied
        FROM daily_stats
        WHERE user_id = $1 AND day = $2
        LIMIT 1"#,
        user_id,
        day,
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(dail_stat)
}
