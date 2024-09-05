use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
}
