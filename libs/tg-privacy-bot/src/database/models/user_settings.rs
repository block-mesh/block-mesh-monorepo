use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub model_name: String,
    pub message_mode: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
