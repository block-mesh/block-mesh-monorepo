use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::aggregate::{Aggregate, AggregateName};

pub async fn get_or_create_aggregate_by_user_and_name(
    transaction: &mut Transaction<'_, Postgres>,
    name: AggregateName,
    user_id: &Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        Aggregate,
        r#"
        INSERT
        INTO aggregates (id, created_at, user_id, name, value, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, name) DO UPDATE SET dummy_updated_at = $6
        RETURNING id, created_at, user_id, name, value, updated_at
        "#,
        id,
        now.clone(),
        user_id,
        name.to_string(),
        value,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(aggregate)
}

#[tracing::instrument(
    name = "get_or_create_aggregate_by_user_and_name",
    skip_all,
    level = "trace",
    ret,
    err
)]
pub(crate) async fn get_or_create_aggregate_by_user_and_name_pool(
    pool: &PgPool,
    name: AggregateName,
    user_id: &Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        Aggregate,
        r#"
        INSERT
        INTO aggregates (id, created_at, user_id, name, value, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, name) DO UPDATE SET dummy_updated_at = $6
        RETURNING id, created_at, user_id, name, value, updated_at
        "#,
        id,
        now.clone(),
        user_id,
        name.to_string(),
        value,
        now
    )
    .fetch_one(pool)
    .await?;
    Ok(aggregate)
}
