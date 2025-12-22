use block_mesh_manager_database_domain::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use time::Date;
use uuid::Uuid;

pub async fn get_daily_stat_by_user_id_and_day(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    day: Date,
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
        created_at,
        uptime,
        updated_at,
        ref_bonus,
        ref_bonus_applied
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
