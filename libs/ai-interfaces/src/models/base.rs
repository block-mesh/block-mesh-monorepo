use crate::error::AiInterfaceError;
use crate::models::anthropic::AnthropicModels;
use crate::models::google::GoogleModels;
use crate::models::meta::MetaModels;
use crate::models::mistral::MistralModels;
use crate::models::open_ai::OpenAiModels;
use crate::models::perplexity::PerplexityModels;
use anyhow::anyhow;
use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgSslMode::Prefer;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum ModelName {
    OpenAi(OpenAiModels),
    Anthropic(AnthropicModels),
    Google(GoogleModels),
    Perplexity(PerplexityModels),
    Meta(MetaModels),
    Mistral(MistralModels),
}

impl From<String> for ModelName {
    fn from(value: String) -> Self {
        Self::try_from_string(value).unwrap_or_else(|_| Self::default())
    }
}

// impl TryFrom<String> for ModelName {
// type Error = AiInterfaceError;
impl ModelName {
    fn try_from_string(value: String) -> Result<Self, AiInterfaceError> {
        if let Ok(open_ai) = OpenAiModels::try_from(value.as_str()) {
            return Ok(Self::OpenAi(open_ai));
        }
        if let Ok(anthropic) = AnthropicModels::try_from(value.as_str()) {
            return Ok(Self::Anthropic(anthropic));
        }
        if let Ok(google) = GoogleModels::try_from(value.as_str()) {
            return Ok(Self::Google(google));
        }
        if let Ok(perplexity) = PerplexityModels::try_from(value.as_str()) {
            return Ok(Self::Perplexity(perplexity));
        }
        if let Ok(meta) = MetaModels::try_from(value.as_str()) {
            return Ok(Self::Meta(meta));
        }
        if let Ok(mistral) = MistralModels::try_from(value.as_str()) {
            return Ok(Self::Mistral(mistral));
        }

        Err(AiInterfaceError::DBError(format!(
            "Invalid ModelName: {}",
            value
        )))
    }
}

impl Display for ModelName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenAi(x) => write!(f, "{}", x),
            Self::Anthropic(x) => write!(f, "{}", x),
            Self::Google(x) => write!(f, "{}", x),
            Self::Perplexity(x) => write!(f, "{}", x),
            _ => write!(f, "{}", Self::default()),
        }
    }
}

impl Default for ModelName {
    fn default() -> Self {
        Self::OpenAi(OpenAiModels::default())
    }
}

impl sqlx::Type<Postgres> for ModelName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for ModelName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for ModelName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        match Self::try_from_string(value) {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(e)),
        }
    }
}
