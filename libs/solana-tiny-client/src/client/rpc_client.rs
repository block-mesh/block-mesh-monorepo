use crate::general::commitment::Commitment;
use crate::methods::get_latest_blockhash::{GetLatestBlockhashInput, GetLatestBlockhashOutput};
use crate::methods::send_transaction::{SendTransactionInput, SendTransactionOutput};
use anyhow::anyhow;
use bincode::serialize;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use solana_sdk::bs58;

#[derive(Debug)]
pub struct RpcClient {
    client: Client,
    default_url: String,
    default_commitment: Commitment,
}

impl RpcClient {
    pub fn new(default_url: String, default_commitment: Commitment) -> Self {
        RpcClient {
            client: Client::new(),
            default_url,
            default_commitment,
        }
    }

    pub fn serialize_and_encode<T>(input: &T) -> anyhow::Result<String>
    where
        T: serde::ser::Serialize,
    {
        let serialized =
            serialize(input).map_err(|e| anyhow!(format!("Serialization failed: {e}")))?;
        let encoded = bs58::encode(serialized).into_string();
        Ok(encoded)
    }

    pub async fn get_latest_blockhash(
        &self,
        url: Option<&str>,
        commitment: Option<Commitment>,
    ) -> anyhow::Result<GetLatestBlockhashOutput> {
        let body = GetLatestBlockhashInput::new(commitment.unwrap_or(self.default_commitment));
        let response = self
            .client
            .post(url.unwrap_or(self.default_url.as_str()))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?
            .json::<GetLatestBlockhashOutput>()
            .await?;
        Ok(response)
    }

    pub async fn send_transaction(
        &self,
        url: Option<&str>,
        serialized_transaction: &str,
    ) -> anyhow::Result<SendTransactionOutput> {
        let body = SendTransactionInput::new(serialized_transaction);
        let response = self
            .client
            .post(url.unwrap_or(self.default_url.as_str()))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?
            .json::<SendTransactionOutput>()
            .await?;
        Ok(response)
    }
}
