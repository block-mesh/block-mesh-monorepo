use sqlx::{Postgres, Transaction};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub async fn count_user_tasks_in_period(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    duration: u64,
) -> anyhow::Result<u64> {
    let since = OffsetDateTime::now_utc() - Duration::seconds(duration as i64);
    let count: Option<i64> = sqlx::query_scalar!(
        r#"
        SELECT
        count(*)
        FROM tasks
        WHERE
        user_id = $1
        AND
        created_at >= $2
        "#,
        user_id,
        since
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default() as u64)
}
