use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
}
