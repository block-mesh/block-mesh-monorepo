use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use std::env;
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_daily_stats", skip_all)]
pub async fn ws_bulk_daily_stats(
    transaction: &mut Transaction<'_, Postgres>,
    user_ids: &Vec<Uuid>,
) -> anyhow::Result<()> {
    if user_ids.is_empty() {
        return Ok(());
    }
    let day = Utc::now().date_naive();
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
            tasks_count = LEAST(tasks_count + {increment}, {upper_limit}),
            updated_at = now()
        WHERE user_id IN ({value_str})
        AND day = '{day}'
        "#,
    );
    let _ = sqlx::query(&query)
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("ws_bulk_daily_stats error {} to run query {}", e, query);
            anyhow!(e)
        })?;
    Ok(())
}
