use crate::database::analytics::inserting_client_analytics::inserting_client_analytics;
use crate::worker::analytics_agg::AnalyticsMessage;
use chrono::Utc;
use serde_json::Value;
use sqlx::{Execute, Postgres, QueryBuilder, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

#[tracing::instrument(
    name = "update_aggregate",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn update_aggregate(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    value: &Value,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    sqlx::query!(
        r#"UPDATE aggregates SET value = $1 , updated_at = $2  WHERE id = $3"#,
        value,
        now,
        id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(*id)
}

pub async fn update_aggregate_bulk(
    mut transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, Value>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let _ = update_aggregate(&mut transaction, pair.0, pair.1).await;
    }
    Ok(())
}

pub async fn inserting_client_analytics_bulk(
    mut transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<Uuid, AnalyticsMessage>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let user_id = pair.0;
        let value = pair.1;
        let _ = inserting_client_analytics(
            &mut transaction,
            user_id,
            &value.depin_aggregator,
            &value.device_type,
        )
        .await;
    }
    Ok(())
}

pub fn update_aggregate_query(value: &Value, id: &Uuid) -> String {
    let mut query = QueryBuilder::<Postgres>::new("UPDATE aggregates SET");
    query.push(" value = '");
    query.push(value);
    query.push("'");
    query.push(" WHERE ");
    query.push(" id = '");
    query.push(id);
    query.push("'");
    query.push(";");
    let q = query.build().sql().into();
    q
}
