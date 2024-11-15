use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;
use sqlx::{Postgres, Transaction};
use tokio::time::Instant;

#[tracing::instrument(
    name = "bulk_uptime_bonus",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub async fn bulk_uptime_bonus(
    transaction: &mut Transaction<'_, Postgres>,
    bonus: f64,
) -> anyhow::Result<PgQueryResult> {
    if bonus.is_nan() || bonus <= 0.0 || bonus.is_infinite() {
        return Err(anyhow!("bulk uptime bonus must be a positive integer"));
    }

    let r = sqlx::query!(
        r#"
        WITH updates (id, value) AS (
        	SELECT id,value FROM aggregates
        	WHERE name = 'Uptime'
        	AND value != 'null'
        	AND updated_at < now() - interval '5 minutes'
		    FOR UPDATE SKIP LOCKED
        )
        UPDATE aggregates
        SET
            value = to_jsonb((COALESCE(NULLIF(aggregates.value, 'null'), '0')::text)::double precision + $1),
            updated_at = now()
        FROM updates
        WHERE aggregates.id = updates.id
        "#,
        bonus
    )
    .execute(&mut **transaction)
    .await?;
    Ok(r)
}
