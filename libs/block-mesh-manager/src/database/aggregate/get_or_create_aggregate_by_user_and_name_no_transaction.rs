use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::aggregate::{Aggregate, AggregateName};

#[tracing::instrument(
    name = "get_or_create_aggregate_by_user_and_name_no_transaction",
    skip(pool),
    ret,
    err
)]
pub(crate) async fn get_or_create_aggregate_by_user_and_name_no_transaction(
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
        INSERT INTO aggregates
        (id, created_at, user_id, name, value)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, name)
        DO UPDATE set user_id = $3
        RETURNING id, created_at, user_id, name, value
        "#,
        id,
        now,
        user_id,
        name.to_string(),
        value
    )
    .fetch_one(pool)
    .await?;
    Ok(aggregate)
}
