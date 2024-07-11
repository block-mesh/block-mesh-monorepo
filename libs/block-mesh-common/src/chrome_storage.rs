use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug)]
pub enum StorageValues {
    BlockMeshUrl,
    Email,
    ApiToken,
    DeviceId,
    Uptime,
    InviteCode,
    DownloadSpeed,
    UploadSpeed,
    LastUpdate,
}

impl Display for StorageValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StorageValues::BlockMeshUrl => "blockmesh_url".to_string(),
            StorageValues::Email => "email".to_string(),
            StorageValues::ApiToken => "blockmesh_api_token".to_string(),
            StorageValues::DeviceId => "device_id".to_string(),
            StorageValues::Uptime => "uptime".to_string(),
            StorageValues::InviteCode => "invite_code".to_string(),
            StorageValues::DownloadSpeed => "download_speed".to_string(),
            StorageValues::UploadSpeed => "upload_speed".to_string(),
            StorageValues::LastUpdate => "last_update".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for StorageValues {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "blockmesh_url" => Ok(StorageValues::BlockMeshUrl),
            "email" => Ok(StorageValues::Email),
            "blockmesh_api_token" => Ok(StorageValues::ApiToken),
            "device_id" => Ok(StorageValues::DeviceId),
            "uptime" => Ok(StorageValues::Uptime),
            "invite_code" => Ok(StorageValues::InviteCode),
            "download_speed" => Ok(StorageValues::DownloadSpeed),
            "upload_speed" => Ok(StorageValues::UploadSpeed),
            "last_update" => Ok(StorageValues::LastUpdate),
            _ => Err("Invalid storage value"),
        }
    }
}

impl TryFrom<&String> for StorageValues {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        StorageValues::try_from(value.as_str())
    }
}

#[derive(Serialize)]
pub enum StorageValue {
    String(String),
    F64(f64),
    I64(i64),
    UUID(Uuid),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StorageMessageType {
    GET,
    SET,
    DELETE,
}

impl Display for StorageMessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            StorageMessageType::GET => f.write_str("GET"),
            StorageMessageType::SET => f.write_str("SET"),
            StorageMessageType::DELETE => f.write_str("DELETE"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageMessage {
    pub r#type: StorageMessageType,
    pub key: String,
    // value: Option<JsValue>,
}
