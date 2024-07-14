use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub enum MessageKey {
    All,
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

impl Serialize for MessageKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for MessageKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MessageKey::All => "all".to_string(),
            MessageKey::BlockMeshUrl => "blockmesh_url".to_string(),
            MessageKey::Email => "email".to_string(),
            MessageKey::ApiToken => "blockmesh_api_token".to_string(),
            MessageKey::DeviceId => "device_id".to_string(),
            MessageKey::Uptime => "uptime".to_string(),
            MessageKey::InviteCode => "invite_code".to_string(),
            MessageKey::DownloadSpeed => "download_speed".to_string(),
            MessageKey::UploadSpeed => "upload_speed".to_string(),
            MessageKey::LastUpdate => "last_update".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for MessageKey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all" => Ok(MessageKey::All),
            "blockmesh_url" => Ok(MessageKey::BlockMeshUrl),
            "email" => Ok(MessageKey::Email),
            "blockmesh_api_token" => Ok(MessageKey::ApiToken),
            "device_id" => Ok(MessageKey::DeviceId),
            "uptime" => Ok(MessageKey::Uptime),
            "invite_code" => Ok(MessageKey::InviteCode),
            "download_speed" => Ok(MessageKey::DownloadSpeed),
            "upload_speed" => Ok(MessageKey::UploadSpeed),
            "last_update" => Ok(MessageKey::LastUpdate),
            _ => Err("Invalid storage value"),
        }
    }
}

impl TryFrom<&String> for MessageKey {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        MessageKey::try_from(value.as_str())
    }
}

#[derive(Deserialize, Debug)]
pub enum MessageValue {
    String(String),
    F64(f64),
    I64(i64),
    UUID(Uuid),
}

impl Serialize for MessageValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageValue::String(s) => serializer.serialize_str(s),
            MessageValue::F64(f) => serializer.serialize_f64(*f),
            MessageValue::I64(i) => serializer.serialize_i64(*i),
            MessageValue::UUID(u) => serializer.serialize_str(&u.to_string()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    GET_ALL,
    GET,
    SET,
    DELETE,
    COPY_TO_CLIPBOARD,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            MessageType::GET_ALL => f.write_str("GET_ALL"),
            MessageType::GET => f.write_str("GET"),
            MessageType::SET => f.write_str("SET"),
            MessageType::DELETE => f.write_str("DELETE"),
            MessageType::COPY_TO_CLIPBOARD => f.write_str("COPY_TO_CLIPBOARD"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum ExtensionStatus {
    LoggedIn,
    Registering,
    LoggedOut,
    WaitingEmailVerification,
}

impl Display for ExtensionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExtensionStatus::LoggedIn => write!(f, "LoggedIn"),
            ExtensionStatus::Registering => write!(f, "Registering"),
            ExtensionStatus::LoggedOut => write!(f, "LoggedOut"),
            ExtensionStatus::WaitingEmailVerification => write!(f, "WaitingEmailVerification"),
        }
    }
}

impl Default for ExtensionStatus {
    fn default() -> Self {
        Self::LoggedOut
    }
}
