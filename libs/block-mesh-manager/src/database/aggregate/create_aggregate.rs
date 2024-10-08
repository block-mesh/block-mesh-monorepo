use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn create_aggregate(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: &AggregateName,
    value: &serde_json::Value,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT
        INTO aggregates (id, created_at, user_id, name, value, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        id,
        now.clone(),
        user_id,
        name.to_string(),
        value,
        now
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
