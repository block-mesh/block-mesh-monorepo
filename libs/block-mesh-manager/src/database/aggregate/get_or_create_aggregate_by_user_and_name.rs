use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::aggregate::{Aggregate, AggregateName};

#[tracing::instrument(
    name = "get_or_create_aggregate_by_user_and_name",
    skip(transaction),
    level = "trace",
    ret,
    err
)]
pub(crate) async fn get_or_create_aggregate_by_user_and_name(
    transaction: &mut Transaction<'_, Postgres>,
    name: AggregateName,
    user_id: Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        Aggregate,
        r#"
        INSERT
        INTO aggregates
        (id, user_id, name, value, created_at, updated_at)
        VALUES
        ($1, $2, $3, CAST( $4 as JSONB ), $5, $6)
        ON CONFLICT (user_id, name) DO UPDATE SET updated_at = $6
        RETURNING id, user_id, name, value, created_at, updated_at
        "#,
        id,
        user_id,
        name.to_string(),
        value,
        now.clone(),
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(aggregate)
}
