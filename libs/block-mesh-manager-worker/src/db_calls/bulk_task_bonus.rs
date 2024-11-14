use sqlx::{Postgres, Transaction};
use tokio::time::Instant;

#[tracing::instrument(name = "bulk_task_bonus", skip(transaction), ret, err, level = "trace")]
pub async fn bulk_task_bonus(
    transaction: &mut Transaction<'_, Postgres>,
    bonus: i32,
) -> anyhow::Result<()> {
    if bonus <= 0 {
        return Ok(());
    }
    let now = Instant::now();
    let r = sqlx::query!(
        r#"
        UPDATE daily_stats ds
            SET	tasks_count = tasks_count + $1
        FROM users u
        WHERE
            ds.user_id = u.id
        	AND ds.status = 'OnGoing'
            AND ds.day = CURRENT_DATE
        "#,
        bonus
    )
    .execute(&mut **transaction)
    .await?;
    let elapsed = now.elapsed();
    tracing::info!(
        "bulk_task_bonus bonus = {} , affected rows = {}, elapsed = {:?}",
        bonus,
        r.rows_affected(),
        elapsed
    );
    Ok(())
}
