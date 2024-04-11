use crate::domain::provider_master_status::ProviderMasterStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct ProxyMaster {
    pub id: Uuid,
    pub address: String,
    pub status: ProviderMasterStatus,
    pub created_at: DateTime<Utc>,
}
