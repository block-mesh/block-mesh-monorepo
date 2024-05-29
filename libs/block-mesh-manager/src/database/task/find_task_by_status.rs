use crate::domain::task::Task;
use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Find task status", skip(transaction), ret, err)]
pub(crate) async fn find_task_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    status: TaskStatus,
) -> anyhow::Result<Option<Task>> {
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT
        id,
        user_id,
        url,
        method,
        headers,
        body,
        assigned_user_id,
        status,
        response_code,
        response_raw,
        created_at
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
