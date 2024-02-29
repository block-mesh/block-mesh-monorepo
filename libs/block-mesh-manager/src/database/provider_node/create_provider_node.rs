use crate::domain::provider_node_status::ProviderNodeStatus;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create ProviderNode", skip(transaction), ret, err)]
pub async fn create_user(
    transaction: &mut Transaction<'_, Postgres>,
    address: &str,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let uuid = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO provider_nodes (id, created_at, address, status) VALUES ($1, $2, $3, $4)"#,
        uuid,
        now,
        address,
        ProviderNodeStatus::Online.to_string(),
    )
    .execute(&mut **transaction)
    .await
    .map_err(|e| {
        let msg = format!("Failed to create provider_node: {:?}", e);
        tracing::error!("{}", msg);
        anyhow!(msg)
    })?;
    Ok(())
}
