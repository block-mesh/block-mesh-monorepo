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
        UPDATE
        daily_stats
        SET status = $1
        WHERE id IN (
            SELECT
            id
            FROM daily_stats
            WHERE
                day < CURRENT_DATE
            AND
                status = $2
            LIMIT $3
        )
        "#,
        "Finalized".to_string(),
        "OnGoing".to_string(),
        limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
