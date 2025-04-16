use crate::utils::rand::rand_factor;
use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;
use sqlx::{Postgres, Transaction};

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
    let r_limit = rand_factor(86400);
    let r = sqlx::query!(
        // r#"
        // WITH updates (id, value) AS (
        // 	SELECT id,value FROM aggregates
        // 	WHERE name = 'Uptime'
        // 	AND value != 'null'
        // 	AND updated_at < now() - interval '5 minutes'
        //     FOR UPDATE SKIP LOCKED
        // )
        // UPDATE aggregates
        // SET
        //     value = to_jsonb((COALESCE(NULLIF(aggregates.value, 'null'), '0')::text)::double precision + $1),
        //     updated_at = now()
        // FROM updates
        // WHERE aggregates.id = updates.id
        // "#,
        r#"
            UPDATE daily_stats_on_going ds
                SET
                    uptime = GREATEST(uptime, LEAST(uptime + $1, $2)),
                    uptime_bonus = GREATEST(uptime_bonus, LEAST(uptime_bonus + $1, $2)),
                    updated_at = now()
            FROM aggregates_uptime a
            WHERE
                ds.user_id = a.user_id
                AND a.updated_at >= NOW() - INTERVAL '15 minutes'
                AND ds.day = CURRENT_DATE
                AND ds.uptime < $2
        "#,
        bonus,
        86400.0 + r_limit as f64
    )
    .execute(&mut **transaction)
    .await?;
    Ok(r)
}
