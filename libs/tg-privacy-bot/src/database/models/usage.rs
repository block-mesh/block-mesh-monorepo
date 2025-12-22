use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub usage_limit: i64,
    pub usage: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub day: Date,
}

impl Usage {
    pub fn over_limit(&self) -> bool {
        self.usage >= self.usage_limit
    }
}
