use crate::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Daily Stats By User Id", skip(transaction), ret, err)]
pub(crate) async fn get_daily_stats_by_user_id(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<DailyStat>> {
    let daily_stats = sqlx::query_as!(
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
        updated_at
        FROM daily_stats
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(daily_stats)
}
