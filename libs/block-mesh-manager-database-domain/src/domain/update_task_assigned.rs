use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_task_assigned", skip_all)]
pub async fn update_task_assigned(
    transaction: &mut Transaction<'_, Postgres>,
    task_id: Uuid,
    user_id: Uuid,
    status: TaskStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE
        tasks
        SET
        assigned_user_id = $1,
        status = $2
        WHERE
        id = $3"#,
        user_id,
        status.to_string(),
        task_id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
