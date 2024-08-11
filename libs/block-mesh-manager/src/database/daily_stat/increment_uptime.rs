use serde_json::Value;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

#[tracing::instrument(
    name = "increment_uptime",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn increment_uptime(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    uptime: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE daily_stats SET uptime = uptime + $1 WHERE id = $2"#,
        uptime,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

pub async fn update_daily_stat_uptime_bulk(
    mut transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, Value>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let _ = increment_uptime(
            &mut transaction,
            pair.0,
            pair.1.as_f64().unwrap_or_default(),
        )
        .await;
    }
    Ok(())
}
