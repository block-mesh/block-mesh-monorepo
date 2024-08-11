use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UsersIp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ip_id: Uuid,
}
