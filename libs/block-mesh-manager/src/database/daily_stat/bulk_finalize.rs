use crate::domain::daily_stat::DailyStatStatus;
use chrono::{Duration, Utc};
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "bulk_finalize", skip(transaction), ret, err, level = "trace")]
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
            WHERE day < $2 and status = $3
            LIMIT 10000
        )
        "#,
        DailyStatStatus::Finalized.to_string(),
        DailyStatStatus::OnGoing.to_string(),
        day
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
