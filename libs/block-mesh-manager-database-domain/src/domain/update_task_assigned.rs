use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_task_assigned", skip_all)]
pub async fn update_task_assigned(
    transaction: &mut Transaction<'_, Postgres>,
    task_id: Uuid,
    assigned_user_id: Uuid,
    status: TaskStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        // r#"
        // UPDATE
        // tasks
        // SET
        // assigned_user_id = $1,
        // status = $2
        // WHERE
        // id = $3"#,
        r#"
WITH locked_task AS (
    SELECT id
    FROM tasks
    WHERE id = $3
    FOR UPDATE SKIP LOCKED
)
UPDATE tasks
SET
    assigned_user_id = $1,
    status = $2
WHERE
    id IN (SELECT id FROM locked_task)
    "#,
        assigned_user_id,
        status.to_string(),
        task_id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
