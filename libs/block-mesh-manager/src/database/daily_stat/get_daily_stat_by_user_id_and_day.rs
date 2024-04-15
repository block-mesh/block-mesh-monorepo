use crate::domain::daily_stat::DailyStat;
use chrono::NaiveDate;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Daily Stat By UserId and Day", skip(transaction), ret, err)]
pub(crate) async fn get_daily_stat_by_user_id_and_day(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    day: NaiveDate,
) -> anyhow::Result<Option<DailyStat>> {
    let daily_stat = sqlx::query_as!(
        DailyStat,
        r#"
        SELECT
        id,
        user_id,
        tasks_count,
        status,
        day,
        created_at
        FROM daily_stats
        WHERE user_id = $1 and day = $2
        "#,
        user_id,
        day
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(daily_stat)
}
