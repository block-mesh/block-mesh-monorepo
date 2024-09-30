use block_mesh_common::constants::DeviceType;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_or_create_analytics", skip_all)]
pub async fn get_or_create_analytics(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    depin_aggregator: &str,
    device_type: &DeviceType,
    version: &str,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT
        INTO analytics
        (user_id, depin_aggregator, device_type, created_at, updated_at, id, version)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_id, depin_aggregator) DO UPDATE SET updated_at = $5
    "#,
        user_id,
        depin_aggregator,
        device_type.to_string(),
        now.clone(),
        now,
        id,
        version
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
