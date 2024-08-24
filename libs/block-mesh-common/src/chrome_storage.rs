use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::option::Option;
use std::str::FromStr;
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
    WalletAddress,
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
            MessageKey::WalletAddress => "wallet_address".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for MessageKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_matches('"');
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
            "wallet_address" => Ok(MessageKey::WalletAddress),
            _ => Err(format!("Invalid MessageKey value {}", value)),
        }
    }
}

impl TryFrom<&String> for MessageKey {
    type Error = String;

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

impl TryFrom<&String> for MessageType {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        MessageType::try_from(value.as_str())
    }
}

impl TryFrom<&str> for MessageType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_matches('"');
        match value {
            "GET_ALL" => Ok(MessageType::GET_ALL),
            "GET" => Ok(MessageType::GET),
            "SET" => Ok(MessageType::SET),
            "DELETE" => Ok(MessageType::DELETE),
            "COPY_TO_CLIPBOARD" => Ok(MessageType::COPY_TO_CLIPBOARD),
            _ => Err(format!("Invalid MessageType: {}", value)),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum AuthStatus {
    LoggedIn,
    Registering,
    LoggedOut,
    WaitingEmailVerification,
}

impl Display for AuthStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthStatus::LoggedIn => write!(f, "LoggedIn"),
            AuthStatus::Registering => write!(f, "Registering"),
            AuthStatus::LoggedOut => write!(f, "LoggedOut"),
            AuthStatus::WaitingEmailVerification => write!(f, "WaitingEmailVerification"),
        }
    }
}

impl Default for AuthStatus {
    fn default() -> Self {
        Self::LoggedOut
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMessage {
    pub msg_type: MessageType,
    pub key: MessageKey,
    pub value: Option<MessageValue>,
}

impl TryFrom<&str> for MessageValue {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_matches('"');
        if let Ok(i) = i64::from_str(value) {
            return Ok(MessageValue::I64(i));
        }
        if let Ok(f) = f64::from_str(value) {
            return Ok(MessageValue::F64(f));
        }
        if let Ok(uuid) = Uuid::parse_str(value) {
            return Ok(MessageValue::UUID(uuid));
        }
        Ok(MessageValue::String(value.to_string()))
    }
}

impl TryFrom<&String> for MessageValue {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        MessageValue::try_from(value.as_str())
    }
}

impl TryFrom<Value> for PostMessage {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let value = if value.is_string() {
            if let Ok(d) = serde_json::from_str::<Value>(value.as_str().unwrap()) {
                d
            } else {
                value
            }
        } else {
            value
        };
        if let Some(object) = value.as_object() {
            let key = object.get("key").ok_or("Missing key")?.to_string();
            let msg_type = object
                .get("msg_type")
                .ok_or("Missing msg_type")?
                .to_string();
            let value = object.get("value").ok_or("Missing value")?.to_string();

            let key = MessageKey::try_from(&key)?;
            let msg_type = MessageType::try_from(&msg_type)?;
            let value = MessageValue::try_from(&value)?;

            Ok(PostMessage {
                key,
                msg_type,
                value: Option::from(value),
            })
        } else {
            Err(format!(
                "Failed to convert value to post message => {}",
                value
            ))
        }
    }
}
