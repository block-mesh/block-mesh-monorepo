use crate::database::models::message_mode::MessageMode;
use ai_interface::models::base::ModelName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub model_name: ModelName,
    pub message_mode: MessageMode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
