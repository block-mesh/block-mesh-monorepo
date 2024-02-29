use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::{Debug, Display};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProviderNodeStatus {
    Online,
    Offline,
    Invalid,
}

impl sqlx::Type<Postgres> for ProviderNodeStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for ProviderNodeStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for ProviderNodeStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

impl Display for ProviderNodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProviderNodeStatus::Online => "Online".to_string(),
            ProviderNodeStatus::Offline => "Offline".to_string(),
            ProviderNodeStatus::Invalid => "Invalid".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for ProviderNodeStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Online" => ProviderNodeStatus::Online,
            "Offline" => ProviderNodeStatus::Offline,
            _ => ProviderNodeStatus::Invalid,
        }
    }
}
