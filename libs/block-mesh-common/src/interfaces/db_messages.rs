use crate::constants::DeviceType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggregateMessage {
    pub id: Uuid,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalyticsMessage {
    pub user_id: Uuid,
    pub depin_aggregator: String,
    pub device_type: DeviceType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyStatMessage {
    pub id: Uuid,
    pub uptime: f64,
}
