use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct InviteCode {
    pub id: Uuid,
    pub invite_code: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
