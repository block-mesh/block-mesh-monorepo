use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub tg_id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,
}
