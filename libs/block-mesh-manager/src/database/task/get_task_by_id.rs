use crate::domain::task::Task;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Tasks By User Id", skip(transaction), ret, err)]
pub(crate) async fn get_task_by_user_id(
    transaction: &mut Transaction<'_, Postgres>,
    task_id: &Uuid,
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
        WHERE id = $1
        "#,
        task_id
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(task)
}
