use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_create_daily_stats", skip_all, err)]
pub async fn ws_bulk_create_daily_stats(
    transaction: &mut Transaction<'_, Postgres>,
    user_ids: &[Uuid],
) -> anyhow::Result<()> {
    tracing::info!("ws_bulk_create_daily_stats starting");
    if user_ids.is_empty() {
        return Ok(());
    }
    let now = Utc::now();
    let day = now.date_naive();
    let values: Vec<String> = user_ids
        .iter()
        .map(|user_id| {
            format!(
                "('{}'::uuid, '{}'::uuid, 'OnGoing', 0, '{}', '{}'::timestamptz, 0, '{}'::timestamptz)",
                Uuid::new_v4(),
                user_id,
                day,
                now.to_rfc3339(),
                now.to_rfc3339()
            )
        })
        .collect();
    let value_str = values.join(",");
    let query = format!(
        r#"
        INSERT
        INTO daily_stats
        (id, user_id, status, tasks_count, day, created_at, uptime, updated_at)
        VALUES {value_str}
        ON CONFLICT(user_id, day) DO UPDATE SET  updated_at = '{now}'::timestamptz
        "#,
    );
    let r = sqlx::query(&query)
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!(
                "ws_bulk_create_daily_stats error {} to run query {}",
                e,
                query
            );
            anyhow!(e)
        })?;
    tracing::info!(
        "ws_bulk_create_daily_stats finished rows_affected = {}",
        r.rows_affected()
    );
    Ok(())
}
