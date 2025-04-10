use chrono::{DateTime, Utc};
use database_utils::utils::option_uuid::OptionUuid;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Aggregate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: AggregateName,
    pub value: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct AggregateTmp {
    pub id: OptionUuid,
    pub user_id: OptionUuid,
    pub name: Option<String>,
    pub value: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AggregateName {
    UFBots,
    Twitter,
    FounderTwitter,
    XenoTwitter,
    WootzAppTwitter,
    Uptime,
    Download,
    Upload,
    Latency,
    Tasks,
    Invalid,
    CronReports,
    WalletChange,
    Wootz,
    InteractiveExt,
    FrodoBots,
    SamIsMoving,
    BitRobot,
    BitRobotNetwork,
    RobotsDotFun,
}

impl Display for AggregateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RobotsDotFun => write!(f, "RobotsDotFun"),
            Self::BitRobotNetwork => write!(f, "BitRobotNetwork"),
            Self::BitRobot => write!(f, "BitRobot"),
            Self::SamIsMoving => write!(f, "SamIsMoving"),
            Self::FrodoBots => write!(f, "FrodoBots"),
            Self::UFBots => write!(f, "UFBots"),
            Self::Twitter => write!(f, "Twitter"),
            Self::FounderTwitter => write!(f, "FounderTwitter"),
            Self::XenoTwitter => write!(f, "XenoTwitter"),
            Self::WootzAppTwitter => write!(f, "WootzAppTwitter"),
            Self::Uptime => write!(f, "Uptime"),
            Self::Download => write!(f, "Download"),
            Self::Upload => write!(f, "Upload"),
            Self::Latency => write!(f, "Latency"),
            Self::Tasks => write!(f, "Tasks"),
            Self::Invalid => write!(f, "Invalid"),
            Self::CronReports => write!(f, "CronReports"),
            Self::WalletChange => write!(f, "WalletChange"),
            Self::Wootz => write!(f, "Wootz"),
            Self::InteractiveExt => write!(f, "InteractiveExt"),
        }
    }
}

impl From<Option<String>> for AggregateName {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from(s),
            None => Self::Invalid,
        }
    }
}
impl From<String> for AggregateName {
    fn from(s: String) -> Self {
        if s.starts_with(&Self::WalletChange.to_string()) {
            return Self::WalletChange;
        }
        match s.as_str() {
            "RobotsDotFun" => Self::RobotsDotFun,
            "BitRobotNetwork" => Self::BitRobotNetwork,
            "BitRobot" => Self::BitRobot,
            "SamIsMoving" => Self::SamIsMoving,
            "FrodoBots" => Self::FrodoBots,
            "UFBots" => Self::UFBots,
            "WootzAppTwitter" => Self::WootzAppTwitter,
            "XenoTwitter" => Self::XenoTwitter,
            "FounderTwitter" => Self::FounderTwitter,
            "Twitter" => Self::Twitter,
            "Uptime" => Self::Uptime,
            "Download" => Self::Download,
            "Upload" => Self::Upload,
            "Latency" => Self::Latency,
            "Tasks" => Self::Tasks,
            "CronReports" => Self::CronReports,
            "WalletChange" => Self::WalletChange,
            "Wootz" => Self::Wootz,
            "InteractiveExt" => Self::InteractiveExt,
            _ => Self::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for AggregateName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for AggregateName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for AggregateName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
