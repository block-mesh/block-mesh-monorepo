use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "delete_uptime_report_by_time",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn delete_uptime_report_by_time(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    seconds: i64,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let diff = now - Duration::seconds(seconds);
    sqlx::query!(
        r#"
        DELETE
            FROM uptime_reports
        WHERE
            created_at < $1
        AND
            user_id = $2
        LIMIT 10000
        "#,
        diff,
        user_id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
