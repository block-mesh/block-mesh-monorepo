use crate::domain::task::Task;
use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Find task excluded user id and status",
    skip(transaction),
    ret,
    err
)]
pub(crate) async fn find_task_by_excluded_user_id_and_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
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
        created_at,
        retries_count,
        country,
        ip,
        asn,
        colo,
        response_time
        FROM tasks
        WHERE user_id != $1 and status = $2
        LIMIT 1
        "#,
        user_id,
        status.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(task)
}
