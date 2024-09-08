use crate::domain::task::GetTask;
use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(
    name = "Find task status",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn find_task_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    status: TaskStatus,
) -> anyhow::Result<Option<GetTask>> {
    let task = sqlx::query_as!(
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
        LIMIT 1
        "#,
        status.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(task)
}
