use chrono::{DateTime, Utc};
use secret::Secret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: Secret<String>,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
}
