use crate::constants::DeviceType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DBMessageTypes {
    UsersIpMessage,
    AggregateMessage,
    AggregateAddToMessage,
    AnalyticsMessage,
    DailyStatMessage,
    AggregateSetToMessage,
    CreateDailyStatMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DBMessage {
    UsersIpMessage(UsersIpMessage),
    AggregateMessage(AggregateMessage),
    AggregateAddToMessage(AggregateAddToMessage),
    AnalyticsMessage(AnalyticsMessage),
    DailyStatMessage(DailyStatMessage),
    CreateDailyStatMessage(CreateDailyStatMessage),
    AggregateSetToMessage(AggregateSetToMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AggregateMessage {
    pub id: Uuid,
    pub value: Value,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AggregateAddToMessage {
    pub user_id: Uuid,
    pub name: String,
    pub value: Value,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AggregateSetToMessage {
    pub user_id: Uuid,
    pub name: String,
    pub value: Value,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnalyticsMessage {
    pub user_id: Uuid,
    pub depin_aggregator: String,
    pub device_type: DeviceType,
    pub version: String,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InvalidateApiCache {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DailyStatMessage {
    pub id: Uuid,
    pub uptime: f64,
    pub msg_type: DBMessageTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateDailyStatMessage {
    pub user_id: Uuid,
    pub msg_type: DBMessageTypes,
}
