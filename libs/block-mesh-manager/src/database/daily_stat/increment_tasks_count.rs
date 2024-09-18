use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn increment_tasks_count(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE daily_stats SET tasks_count = tasks_count + 1 WHERE id = $1"#,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
