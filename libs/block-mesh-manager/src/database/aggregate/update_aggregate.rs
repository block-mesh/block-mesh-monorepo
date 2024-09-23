use chrono::Utc;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_aggregate(
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
