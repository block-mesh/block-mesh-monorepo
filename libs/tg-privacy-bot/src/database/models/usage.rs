use chrono::{DateTime, NaiveDate, Utc};
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
    pub day: NaiveDate,
}

impl Usage {
    pub fn over_limit(&self) -> bool {
        self.usage > self.usage_limit
    }
}
