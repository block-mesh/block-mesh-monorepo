use sqlx::{Postgres, Transaction};
use time::{Duration, OffsetDateTime};

#[tracing::instrument(name = "finalize_cleanup", skip_all, err)]
pub async fn finalize_cleanup(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
    let day = now.date() - Duration::days(2);
    sqlx::query!(
        r#"
        DELETE
        FROM daily_stats_on_going d
        WHERE d.day = $1
        AND d.status = 'OnGoing'
        AND d.day < CURRENT_DATE - INTERVAL '1 day'
        AND EXISTS (
            SELECT 1
            FROM daily_stats_finalized f
            WHERE f.day = $1
            AND f.status = 'Finalized'
            )
        "#,
        day
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
