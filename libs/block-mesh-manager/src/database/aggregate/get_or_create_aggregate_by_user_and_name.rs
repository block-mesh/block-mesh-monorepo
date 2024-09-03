use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::aggregate::{Aggregate, AggregateName};

#[tracing::instrument(
    name = "get_or_create_aggregate_by_user_and_name",
    skip_all,
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
         WITH
            extant AS (
                SELECT id, user_id, name, value, created_at, updated_at FROM aggregates WHERE user_id = $3 AND name = $4
            ),
            inserted AS (
                INSERT INTO aggregates (id , created_at, user_id, name, value, updated_at)
                SELECT $1, $2, $3, $4,  CAST( $5 as JSONB ), $6
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id, user_id, name, value, created_at, updated_at
            )
        SELECT id, user_id, name, value, created_at, updated_at FROM inserted
        UNION ALL
        SELECT id, user_id, name, value, created_at, updated_at FROM extant
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
    user_id: Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        Aggregate,
         r#"
         WITH
            extant AS (
                SELECT id, user_id, name, value, created_at, updated_at FROM aggregates WHERE user_id = $3 AND name = $4
            ),
            inserted AS (
                INSERT INTO aggregates (id , created_at, user_id, name, value, updated_at)
                SELECT $1, $2, $3, $4,  CAST( $5 as JSONB ), $6
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id, user_id, name, value, created_at, updated_at
            )
        SELECT id, user_id, name, value, created_at, updated_at FROM inserted
        UNION ALL
        SELECT id, user_id, name, value, created_at, updated_at FROM extant
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
