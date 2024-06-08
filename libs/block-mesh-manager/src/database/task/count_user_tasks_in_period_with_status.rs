use crate::domain::task::TaskStatus;
use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Count User Tasks in Period of Time with Status",
    skip(transaction),
    ret,
    err
)]
pub async fn count_user_tasks_in_period_with_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    duration: u64,
    status: TaskStatus,
) -> anyhow::Result<i64> {
    let since = Utc::now() - Duration::seconds(duration as i64);
    let count: Option<i64> = sqlx::query_scalar!(
        r#"
        SELECT
        count(*)
        FROM tasks
        WHERE
        user_id = $1
        AND
        created_at >= $2
        AND
        status = $3
        "#,
        user_id,
        since,
        status.to_string()
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default())
}
