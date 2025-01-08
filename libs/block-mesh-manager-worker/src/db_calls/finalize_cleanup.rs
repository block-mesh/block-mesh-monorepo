use chrono::Utc;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "finalize_cleanup", skip_all, err)]
pub async fn finalize_cleanup(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let now = Utc::now();
    let day = now.date_naive();
    sqlx::query!(
        r#"
        DELETE
        FROM daily_stats_on_going
        WHERE
        id IN (
            SELECT
                DISTINCT daily_stats_on_going.id
            FROM daily_stats_on_going
			INNER JOIN daily_stats_finalized ON daily_stats_on_going.day = daily_stats_finalized.day
		WHERE
			daily_stats_on_going.day = $1
			AND daily_stats_on_going.status = 'OnGoing'
		)
        "#,
        day
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
