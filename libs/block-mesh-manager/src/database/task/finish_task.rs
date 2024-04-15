use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "finish_task", skip(transaction, response_raw), ret, err)]
pub(crate) async fn finish_task(
    transaction: &mut Transaction<'_, Postgres>,
    task_id: Uuid,
    response_code: Option<i32>,
    response_raw: Option<String>,
    status: TaskStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE
        tasks
        SET
        response_code = $1,
        response_raw = $2,
        status = $3
        WHERE id = $4"#,
        response_code,
        response_raw,
        status.to_string(),
        task_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
