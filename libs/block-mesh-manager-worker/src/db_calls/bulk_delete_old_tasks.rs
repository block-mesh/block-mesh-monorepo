use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};
use std::env;

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
    let bulk_delete_limit = env::var("BULK_DELETE_LIMIT")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    sqlx::query!(
        r#"
        DELETE FROM tasks WHERE id IN (SELECT id from tasks WHERE created_at < $1 LIMIT $2 FOR UPDATE SKIP LOCKED)
        "#,
        date,
        bulk_delete_limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
