use sqlx::{Postgres, Transaction};
use std::env;

#[tracing::instrument(name = "bulk_finalize", skip_all, err)]
pub async fn bulk_finalize(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let limit = env::var("FINALIZE_LIMIT")
        .unwrap_or("100000".to_string())
        .parse()
        .unwrap_or(100_000);
    sqlx::query!(
        r#"
        WITH to_finalize AS (
            SELECT id
            FROM daily_stats_on_going
            WHERE DAY < CURRENT_DATE
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE daily_stats
        SET status = 'Finalized'
        WHERE id IN (SELECT id FROM to_finalize)
        "#,
        limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
