use crate::domain::task::GetTask;
use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Find task assigned to user",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn find_task_assigned_to_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
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
        WHERE status = $1 AND assigned_user_id = $2
        LIMIT 1
        "#,
        TaskStatus::Assigned.to_string(),
        user_id
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(task)
}
