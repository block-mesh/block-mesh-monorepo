use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};

#[tracing::instrument(
    name = "delete_uptime_report_by_time_for_all",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn delete_uptime_report_by_time_for_all(
    transaction: &mut Transaction<'_, Postgres>,
    seconds: i64,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let diff = now - Duration::seconds(seconds);
    sqlx::query!(
        r#"
        DELETE FROM uptime_reports
        WHERE id in (
            SELECT id FROM uptime_reports
                WHERE
                    created_at < $1
                LIMIT 10000
        )
        "#,
        diff,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
