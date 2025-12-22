use crate::domain::provider_master_status::ProviderMasterStatus;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create_proxy_master(
    transaction: &mut Transaction<'_, Postgres>,
    address: &str,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
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
