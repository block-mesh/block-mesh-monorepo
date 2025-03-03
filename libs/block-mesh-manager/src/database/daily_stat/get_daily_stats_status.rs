use block_mesh_manager_database_domain::domain::daily_stat::DailyStatStatus;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DailyStatCount {
    pub true_count: i64,
    pub false_count: i64,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DailyStatCountTmp {
    pub true_count: Option<i64>,
    pub false_count: Option<i64>,
}

pub async fn get_daily_stats_ref_status_by_user_id(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<DailyStatCount> {
    let daily_stats_count = sqlx::query_as!(
        DailyStatCountTmp,
        r#"
        SELECT
            COUNT(*) FILTER  (WHERE ref_bonus_applied = TRUE)  AS true_count,
            COUNT(*) FILTER  (WHERE ref_bonus_applied = FALSE) AS false_count
        FROM daily_stats
        WHERE
            user_id = $1
        AND
            status = $2
        "#,
        user_id,
        DailyStatStatus::Finalized.to_string()
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(DailyStatCount {
        true_count: daily_stats_count.true_count.unwrap_or_default(),
        false_count: daily_stats_count.false_count.unwrap_or_default(),
    })
}
