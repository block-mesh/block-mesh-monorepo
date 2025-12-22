use sqlx::{Postgres, Transaction};
use time::{Duration, OffsetDateTime};

#[allow(dead_code)]
pub async fn delete_uptime_report_by_time_for_all(
    transaction: &mut Transaction<'_, Postgres>,
    seconds: i64,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
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
