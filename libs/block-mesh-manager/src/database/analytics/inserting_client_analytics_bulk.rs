use crate::database::analytics::inserting_client_analytics::inserting_client_analytics;
use crate::worker::analytics_agg::AnalyticsMessage;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn inserting_client_analytics_bulk(
    transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, AnalyticsMessage>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let user_id = pair.0;
        let value = pair.1;
        let _ = inserting_client_analytics(
            transaction,
            user_id,
            &value.depin_aggregator,
            &value.device_type,
        )
        .await;
    }
    Ok(())
}
