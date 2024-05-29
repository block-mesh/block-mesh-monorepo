use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::ops::Deref;
use uuid::Uuid;
// https://stackoverflow.com/questions/75242725/rust-sqlx-try-from-optionuuid
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionUuid(pub Option<Uuid>);

impl sqlx::Type<Postgres> for OptionUuid {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for OptionUuid {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let value = match self.0 {
            Some(value) => value.to_string(),
            None => "null".to_string(),
        };
        <String as sqlx::Encode<Postgres>>::encode(value, buf)
    }
}

impl sqlx::Decode<'_, Postgres> for OptionUuid {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <Uuid as Decode<Postgres>>::decode(value)?;
        Ok(Self(Some(value)))
    }
}

impl TryFrom<String> for OptionUuid {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = if value.is_empty() {
            None
        } else {
            let uuid = Uuid::try_parse(&value)?;
            Some(uuid)
        };
        Ok(Self(inner))
    }
}

impl Deref for OptionUuid {
    type Target = Option<Uuid>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<OptionUuid>> for OptionUuid {
    fn from(option: Option<OptionUuid>) -> Self {
        option.unwrap_or(OptionUuid(None))
    }
}
