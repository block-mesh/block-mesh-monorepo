use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_uptime", skip_all)]
pub async fn ws_bulk_uptime(
    transaction: &mut Transaction<'_, Postgres>,
    user_ids: &[Uuid],
    diff: f64,
) -> anyhow::Result<()> {
    if user_ids.is_empty() {
        return Ok(());
    }
    let values: Vec<String> = user_ids
        .iter()
        .map(|id| format!("'{}'::uuid", id))
        .collect();
    let value_str = values.join(",");
    let query = format!(
        r#"
        WITH updates (id, value) AS (SELECT id, value FROM aggregates WHERE name = 'Uptime' AND user_id in ({value_str}))
        UPDATE aggregates
        SET
            value = to_jsonb(COALESCE(NULLIF(aggregates.value::text, 'null')::double precision, {diff})),
            updated_at = now()
        FROM updates
        WHERE aggregates.id = updates.id;
        "#
    );
    let _ = sqlx::query(&query)
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("ws_bulk_uptime error {} failed to run query {}", e, query);
            anyhow!(e)
        })?;
    Ok(())
}
