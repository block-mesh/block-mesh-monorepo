use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Default)]
pub enum PerkName {
    Everlyn,
    Intract,
    Wallet,
    Twitter,
    FounderTwitter,
    XenoTwitter,
    WootzTwitter,
    #[default]
    Invalid,
    ProofOfHumanity,
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
    Grandmaster,
    Legend,
    UFBots,
    FrodoBots,
    SamIsMoving,
    BitRobot,
    BitRobotNetwork,
    RobotsDotFun,
    PerceptronNTWK,
    MRRydon,
    Peter_thoc,
}

impl Display for PerkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Everlyn => write!(f, "Everlyn_ai"),
            Self::PerceptronNTWK => write!(f, "PerceptronNTWK"),
            Self::MRRydon => write!(f, "MRRydon"),
            Self::Peter_thoc => write!(f, "Peter_thoc"),
            Self::RobotsDotFun => write!(f, "RobotsDotFun"),
            Self::BitRobot => write!(f, "BitRobot"),
            Self::BitRobotNetwork => write!(f, "BitRobotNetwork"),
            Self::SamIsMoving => write!(f, "SamIsMoving"),
            Self::FrodoBots => write!(f, "FrodoBots"),
            Self::UFBots => write!(f, "UFBots"),
            Self::Intract => write!(f, "intract"),
            Self::WootzTwitter => write!(f, "wootz_twitter"),
            Self::Wallet => write!(f, "wallet"),
            Self::Twitter => write!(f, "twitter"),
            Self::FounderTwitter => write!(f, "founder_twitter"),
            Self::XenoTwitter => write!(f, "xeno_twitter"),
            Self::ProofOfHumanity => write!(f, "proof_of_humanity"),
            Self::Novice => write!(f, "novice"),
            Self::Apprentice => write!(f, "apprentice"),
            Self::Journeyman => write!(f, "journeyman"),
            Self::Expert => write!(f, "expert"),
            Self::Master => write!(f, "master"),
            Self::Grandmaster => write!(f, "grandmaster"),
            Self::Legend => write!(f, "legend"),
            Self::Invalid => write!(f, "invalid"),
        }
    }
}

impl From<String> for PerkName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Everlyn_ai" => Self::Everlyn,
            "PerceptronNTWK" => Self::PerceptronNTWK,
            "MRRydon" => Self::MRRydon,
            "Peter_thoc" => Self::Peter_thoc,
            "RobotsDotFun" => Self::RobotsDotFun,
            "BitRobotNetwork" => Self::BitRobotNetwork,
            "BitRobot" => Self::BitRobot,
            "SamIsMoving" => Self::SamIsMoving,
            "FrodoBots" => Self::FrodoBots,
            "UFBots" => Self::UFBots,
            "intract" => Self::Intract,
            "wootz_twitter" => Self::WootzTwitter,
            "wallet" => Self::Wallet,
            "twitter" => Self::Twitter,
            "founder_twitter" => Self::FounderTwitter,
            "xeno_twitter" => Self::XenoTwitter,
            "proof_of_humanity" => Self::ProofOfHumanity,
            "novice" => Self::Novice,
            "apprentice" => Self::Apprentice,
            "journeyman" => Self::Journeyman,
            "expert" => Self::Expert,
            "master" => Self::Master,
            "grandmaster" => Self::Grandmaster,
            "legend" => Self::Legend,
            _ => Self::Invalid,
        }
    }
}

impl From<Option<String>> for PerkName {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from(s),
            None => Self::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for PerkName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for PerkName {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for PerkName {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Perk {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub multiplier: f64,
    pub one_time_bonus: f64,
    pub name: PerkName,
    pub data: Value,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerkTmp {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    pub multiplier: Option<f64>,
    pub one_time_bonus: Option<f64>,
    pub name: Option<String>,
    pub data: Option<Value>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}
