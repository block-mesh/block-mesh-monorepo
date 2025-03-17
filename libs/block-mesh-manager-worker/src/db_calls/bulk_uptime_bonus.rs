use crate::utils::rand::rand_factor;
use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;
use sqlx::{Postgres, Transaction};
use std::env;

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
    let bulk_uptime_bonus_query_limit = env::var("BULK_UPTIME_BONUS_QUERY_LIMIT")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap_or(1_000);
    let r_limit = rand_factor(86400);
    let r = sqlx::query!(
        r#"
        WITH limited_rows AS (
            SELECT ds.id
            FROM daily_stats ds
            JOIN aggregates a ON ds.user_id = a.user_id
            WHERE a.name = 'Uptime'
                AND a.updated_at >= NOW() - INTERVAL '2 hour'
                AND ds.status = 'OnGoing'
                AND ds.day = CURRENT_DATE
                AND ds.uptime < $2
            LIMIT $3
        )
        UPDATE daily_stats ds
        SET
            uptime = GREATEST(uptime, LEAST(uptime + $1, $2)),
            uptime_bonus = GREATEST(uptime_bonus, LEAST(uptime_bonus + $1, $2)),
            updated_at = NOW()
        FROM limited_rows lr
        WHERE ds.id = lr.id
            AND ds.status = 'OnGoing'
            AND ds.day = CURRENT_DATE
        "#,
        bonus,
        86400.0 + r_limit as f64,
        bulk_uptime_bonus_query_limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(r)
}
