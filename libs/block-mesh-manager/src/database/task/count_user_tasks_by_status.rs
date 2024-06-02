use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Count User Tasks By Status", skip(transaction), ret, err)]
pub async fn count_user_tasks_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    status: TaskStatus,
) -> anyhow::Result<i64> {
    let count: Option<i64> = sqlx::query_scalar!(
        r#"
        SELECT
        COALESCE(COUNT(*), 0) AS count
        FROM
        (
            SELECT 1
            FROM tasks
            WHERE
            user_id = $1
            AND
            status = $2
        ) as subquery
        "#,
        user_id,
        status.to_string()
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default())
}
