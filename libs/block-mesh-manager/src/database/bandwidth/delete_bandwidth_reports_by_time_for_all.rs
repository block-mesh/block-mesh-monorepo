use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "delete_bandwidth_reports_by_time_for_all",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn delete_bandwidth_reports_by_time_for_all(
    transaction: &mut Transaction<'_, Postgres>,
    seconds: i64,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let diff = now - Duration::seconds(seconds);
    sqlx::query!(
        r#"
        DELETE
            FROM bandwidth_reports
        WHERE
            created_at < $1
        "#,
        diff,
        user_id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
