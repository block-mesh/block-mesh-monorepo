use crate::domain::task::GetTask;
use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};

pub async fn find_users_tasks(
    transaction: &mut Transaction<'_, Postgres>,
    limit: i64,
) -> anyhow::Result<Vec<GetTask>> {
    let tasks = sqlx::query_as!(
        GetTask,
        r#"
        SELECT
        id,
        url,
        method,
        headers,
        body
        FROM tasks
        WHERE status = $1
        LIMIT $2
        "#,
        TaskStatus::Pending.to_string(),
        limit
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(tasks)
}
