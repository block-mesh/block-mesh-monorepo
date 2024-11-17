use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use std::env;
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_daily_stats", skip_all, err)]
pub async fn ws_bulk_daily_stats(
    transaction: &mut Transaction<'_, Postgres>,
    user_ids: &[Uuid],
    diff: f64,
) -> anyhow::Result<()> {
    tracing::info!("ws_bulk_daily_stats starting");
    if user_ids.is_empty() {
        return Ok(());
    }
    let upper_limit = env::var("WS_BULK_DAILY_STATS_UPPER_LIMIT")
        .unwrap_or("50".to_string())
        .parse::<i32>()
        .unwrap_or(50);
    let increment = env::var("WS_BULK_DAILY_STATS_UPPER_INCREMENT")
        .unwrap_or("10".to_string())
        .parse::<i32>()
        .unwrap_or(10);
    let values: Vec<String> = user_ids
        .iter()
        .map(|id| format!("'{}'::uuid", id,))
        .collect();
    let value_str = values.join(",");
    let query = format!(
        r#"
        UPDATE daily_stats
        SET
            tasks_count = GREATEST(tasks_count, LEAST(tasks_count + {increment}, {upper_limit})),
            tasks_count_bonus = GREATEST(tasks_count_bonus, tasks_count_bonus + (LEAST(tasks_count + {increment}, {upper_limit}) - tasks_count)),
            uptime = uptime + {diff},
            uptime_bonus = uptime_bonus + {diff},
            updated_at = now()
        WHERE
            user_id IN ({value_str})
            AND status = 'OnGoing'
            AND day = CURRENT_DATE
        "#,
    );
    let r = sqlx::query(&query)
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!(
                "ws_bulk_daily_stats error {} to run query size {}",
                e,
                user_ids.len()
            );
            anyhow!(e)
        })?;
    tracing::info!(
        "ws_bulk_daily_stats finished rows_affected = {}",
        r.rows_affected()
    );
    Ok(())
}
