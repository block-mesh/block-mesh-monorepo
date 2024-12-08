use crate::utils::rand::rand_factor;
use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;
use sqlx::{Postgres, Transaction};

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
    let r = sqlx::query!(
        r#"
        UPDATE daily_stats ds
            SET
                tasks_count = GREATEST(tasks_count, LEAST(tasks_count + $1, $2)),
                tasks_count_bonus = GREATEST(tasks_count_bonus, tasks_count_bonus + (LEAST(tasks_count + $1, $2) - tasks_count)),
                updated_at = now()
        FROM aggregates a
        WHERE
            ds.user_id = a.user_id
            AND a.name = 'Uptime'
            AND a.updated_at >= NOW() - INTERVAL '2 hour'
            AND ds.status = 'OnGoing'
            AND ds.day = CURRENT_DATE
            AND ds.tasks_count < $2
        "#,
        bonus,
        limit + r_limit
    )
    .execute(&mut **transaction)
    .await?;
    Ok(r)
}
