use crate::database::daily_stat::increment_uptime::increment_uptime;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn update_daily_stat_uptime_bulk(
    transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, f64>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let _ = increment_uptime(transaction, pair.0, *pair.1).await;
    }
    Ok(())
}
