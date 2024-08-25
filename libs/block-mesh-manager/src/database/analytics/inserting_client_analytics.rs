use crate::domain::api_token::ApiTokenStatus;
use block_mesh_common::constants::DeviceType;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Inserting client analytics", skip(transaction), ret, err)]
pub(crate) async fn inserting_client_analytics(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    depin_aggregator: &str,
    device_type: &DeviceType,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let token = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT
        INTO analytics
        (user_id, depin_aggregator, device_type, created_at, updated_at, id)
        VALUES
        ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, depin_aggregator) DO UPDATE SET updated_at = $5
    "#,
        user_id,
        depin_aggregator,
        device_type.to_string(),
        now.clone(),
        now,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
