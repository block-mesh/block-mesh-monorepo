use crate::domain::aggregate::AggregateName;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "create_aggregate", skip(transaction), ret, err)]
pub(crate) async fn create_aggregate(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: &AggregateName,
    value: &serde_json::Value,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO aggregates (id, created_at, user_id, name, value) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        now,
        user_id,
        name.to_string(),
        value
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
