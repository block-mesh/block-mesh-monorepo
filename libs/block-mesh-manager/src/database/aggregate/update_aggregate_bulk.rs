use crate::database::aggregate::update_aggregate::update_aggregate;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn update_aggregate_bulk(
    transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, Value>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let _ = update_aggregate(transaction, pair.0, pair.1).await;
    }
    Ok(())
}
