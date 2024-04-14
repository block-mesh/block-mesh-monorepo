use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RankedUsers {
    pub user_id: Uuid,
    pub total_tasks_count: Option<i64>,
    pub rank: Option<i64>,
}

#[tracing::instrument(name = "Get Users Rank", skip(transaction), ret, err)]
pub(crate) async fn get_users_rank(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<RankedUsers>> {
    let ranks = sqlx::query_as!(
        RankedUsers,
        r#"
        WITH RankedUsers AS (
            SELECT
            user_id,
            SUM(tasks_count) as total_tasks_count,
            RANK() OVER (ORDER BY SUM(tasks_count) DESC) as rank
        FROM daily_stats
        GROUP BY user_id
        )
        SELECT
            user_id,
            total_tasks_count,
            rank
        FROM RankedUsers
        ORDER BY rank;
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(ranks)
}
