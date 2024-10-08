use block_mesh_manager_database_domain::domain::task::GetTask;
use block_mesh_manager_database_domain::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn find_task_assigned_to_user(
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
