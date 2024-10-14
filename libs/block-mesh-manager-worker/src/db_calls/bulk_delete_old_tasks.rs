use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};

#[tracing::instrument(
    name = "bulk_delete_old_tasks",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub async fn bulk_delete_old_tasks(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<()> {
    let date = Utc::now() - Duration::days(1);
    sqlx::query!(
        r#"
        DELETE FROM tasks WHERE created_at < $1
        "#,
        date,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
