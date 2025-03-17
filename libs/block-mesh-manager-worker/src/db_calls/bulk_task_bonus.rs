use crate::utils::rand::rand_factor;
use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;
use sqlx::{Postgres, Transaction};
use std::env;

#[tracing::instrument(name = "bulk_task_bonus", skip(transaction), ret, err, level = "trace")]
pub async fn bulk_task_bonus(
    transaction: &mut Transaction<'_, Postgres>,
    bonus: i32,
    limit: i32,
) -> anyhow::Result<PgQueryResult> {
    if bonus <= 0 || limit <= 0 {
        return Err(anyhow!("bulk_task_bonus called without a limit and bonus"));
    }
    let r_limit = rand_factor(limit);
    let bulk_task_bonus_query_limit = env::var("BULK_TASK_BONUS_QUERY_LIMIT")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap_or(1_000);

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
                AND ds.tasks_count < $2
            LIMIT $3
        )
        UPDATE daily_stats ds
        SET
            tasks_count = GREATEST(tasks_count, LEAST(tasks_count + $1, $2)),
            tasks_count_bonus = GREATEST(tasks_count_bonus, tasks_count_bonus + (LEAST(tasks_count + $1, $2) - tasks_count)),
            updated_at = NOW()
        FROM limited_rows lr
        WHERE ds.id = lr.id
        AND ds.status = 'OnGoing'
        AND ds.day = CURRENT_DATE
        "#,
        bonus,
        limit + r_limit,
        bulk_task_bonus_query_limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(r)
}
