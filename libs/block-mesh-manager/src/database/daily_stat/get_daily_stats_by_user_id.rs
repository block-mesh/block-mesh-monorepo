use block_mesh_manager_database_domain::domain::daily_stat::DailyStat;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_daily_stats_by_user_id(
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
        updated_at,
        ref_bonus,
        ref_bonus_applied
        FROM daily_stats
        WHERE user_id = $1
        ORDER BY day DESC
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(daily_stats)
}
