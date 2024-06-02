use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get User Rank by Task Statsu", skip(transaction), ret, err)]
pub(crate) async fn get_user_rank_by_task_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    status: TaskStatus,
) -> anyhow::Result<i64> {
    let rank: Option<i64> = sqlx::query_scalar!(
        r#"
        SELECT
            RANK() OVER (ORDER BY COALESCE(COUNT(t.id), 0) DESC) AS user_rank
            FROM
                users u
            LEFT JOIN
                tasks t ON u.id = t.assigned_user_id
            AND t.status = $1
            WHERE u.id = $2
        "#,
        status.to_string(),
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(rank.unwrap_or_default())
}
