use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UptimeReport {
    pub id: Uuid,
    pub nonce: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
