use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub usage_limit: i64,
    pub usage: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Usage {
    pub fn over_limit(&self) -> bool {
        // TODO: Need to implement
        false
    }
}
