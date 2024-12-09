use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "bulk_finalize", skip_all, err)]
pub async fn bulk_finalize(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let now = Utc::now() - Duration::days(1);
    let day = now.date_naive();
    sqlx::query!(
        r#"
        UPDATE
        daily_stats
        SET status = $1
        WHERE id IN (
            SELECT
            id
            FROM daily_stats
            WHERE day < $2 AND status = $3
            LIMIT 100000
        )
        "#,
        "Finalized".to_string(),
        day,
        "OnGoing".to_string()
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
