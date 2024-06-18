use crate::database::task::create_task::create_task;
use crate::domain::task::TaskMethod;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Decode, Postgres, Transaction};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum RpcName {
    Helius,
    QuickNode,
    HelloMoon,
    HelloMoonGlobal,
    #[default]
    SolanaLabs,
    Shyft,
    Alchemy,
    Syndica,
    Chainstack,
}

impl RpcName {
    pub fn from_url(url: &str) -> Self {
        if url.starts_with("https://mainnet.helius-rpc.com") {
            RpcName::Helius
        } else if url.contains("quicknode.pro") {
            RpcName::QuickNode
        } else if url.starts_with("https://global.rpc.hellomoon.io") {
            RpcName::HelloMoonGlobal
        } else if url.starts_with("https://rpc.hellomoon.io") {
            RpcName::HelloMoon
            // } else if url.starts_with("https://api.mainnet-beta.solana.com") {
            //     RpcName::SolanaLabs
        } else if url.starts_with("https://rpc.shyft.to") {
            RpcName::Shyft
        } else if url.starts_with("https://solana-mainnet.g.alchemy.com") {
            RpcName::Alchemy
        } else if url.starts_with("https://solana-mainnet.api.syndica.io") {
            RpcName::Syndica
        } else if url.starts_with("https://solana-mainnet.core.chainstack.com") {
            RpcName::Chainstack
        } else {
            RpcName::SolanaLabs
        }
    }
}

impl Display for RpcName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcName::Helius => write!(f, "Helius"),
            RpcName::QuickNode => write!(f, "QuickNode"),
            RpcName::HelloMoon => write!(f, "HelloMoon"),
            RpcName::HelloMoonGlobal => write!(f, "HelloMoonGlobal"),
            RpcName::SolanaLabs => write!(f, "SolanaLabs"),
            RpcName::Shyft => write!(f, "Shyft"),
            RpcName::Alchemy => write!(f, "Alchemy"),
            RpcName::Syndica => write!(f, "Syndica"),
            RpcName::Chainstack => write!(f, "Chainstack"),
        }
    }
}

impl From<String> for RpcName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Helius" => RpcName::Helius,
            "QuickNode" => RpcName::QuickNode,
            "HelloMoon" => RpcName::HelloMoon,
            "HelloMoonGlobal" => RpcName::HelloMoonGlobal,
            "SolanaLabs" => RpcName::SolanaLabs,
            "Shyft" => RpcName::Shyft,
            "Alchemy" => RpcName::Alchemy,
            "Syndica" => RpcName::Syndica,
            "Chainstack" => RpcName::Chainstack,
            _ => RpcName::SolanaLabs,
        }
    }
}

impl sqlx::Type<Postgres> for RpcName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for RpcName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for RpcName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Rpc {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub token: String,
    pub host: String,
    pub name: RpcName,
}

impl Rpc {
    pub async fn create_rpc_task(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        uuid: &Uuid,
    ) -> anyhow::Result<()> {
        let url = match &self.name {
            RpcName::Helius => {
                format!("{}?api-key={}", self.host, self.token)
            }
            RpcName::Shyft => {
                format!("{}?api_key={}", self.host, self.token)
            }
            RpcName::SolanaLabs => self.host.to_string(),
            _ => {
                format!("{}/{}", self.host, self.token)
            }
        };
        let method = TaskMethod::POST;
        let headers = json!(
            {
            "Content-Type": "application/json"
            }
        );
        let body = json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"getLatestBlockhash",
            "params": vec![
                json!({
                    "commitment":"processed"
                })
            ]
        });

        create_task(transaction, uuid, &url, &method, Some(headers), Some(body)).await?;
        Ok(())
    }
}
