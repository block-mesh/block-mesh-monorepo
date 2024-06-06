use crate::domain::task::Task;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Tasks By User Id", skip(transaction), ret, err)]
pub(crate) async fn get_tasks_by_user_id(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<Task>> {
    let tasks = sqlx::query_as!(
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
        retries_count
        FROM tasks
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(tasks)
}
