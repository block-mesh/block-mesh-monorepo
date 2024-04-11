use crate::domain::provider_master_status::ProviderMasterStatus;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Proxy Master", skip(transaction), ret, err)]
pub async fn create_proxy_master(
    transaction: &mut Transaction<'_, Postgres>,
    address: &str,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let uuid = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO proxy_masters (id, created_at, address, status) VALUES ($1, $2, $3, $4)"#,
        uuid,
        now,
        address,
        ProviderMasterStatus::Online.to_string(),
    )
    .execute(&mut **transaction)
    .await
    .map_err(|e| {
        let msg = format!("Failed to create proxy_master: {:?}", e);
        tracing::error!("{}", msg);
        anyhow!(msg)
    })?;
    Ok(())
}
