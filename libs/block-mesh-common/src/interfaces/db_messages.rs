use crate::constants::DeviceType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DBMessageTypes {
    UsersIpMessage,
    AggregateMessage,
    AnalyticsMessage,
    DailyStatMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggregateMessage {
    pub id: Uuid,
    pub value: Value,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalyticsMessage {
    pub user_id: Uuid,
    pub depin_aggregator: String,
    pub device_type: DeviceType,
    pub version: String,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvalidateApiCache {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyStatMessage {
    pub id: Uuid,
    pub uptime: f64,
    pub msg_type: DBMessageTypes,
}
