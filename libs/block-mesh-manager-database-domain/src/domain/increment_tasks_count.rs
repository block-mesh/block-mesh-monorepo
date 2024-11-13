use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "increment_tasks_count", skip_all)]
pub async fn increment_tasks_count(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
    value: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE daily_stats SET tasks_count = tasks_count + $2 WHERE id = $1"#,
        id,
        value
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
