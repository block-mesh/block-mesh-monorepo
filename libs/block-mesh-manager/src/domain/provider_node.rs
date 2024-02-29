use crate::domain::provider_node_status::ProviderNodeStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct ProviderNode {
    pub id: Uuid,
    pub address: String,
    pub status: ProviderNodeStatus,
    pub created_at: DateTime<Utc>,
}
