use crate::general::commitment::Commitment;
use crate::methods::getlatestblockhash::{GetLatestBlockhashInput, GetLatestBlockhashOutput};
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;

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

    pub async fn get_latest_blockhash(
        &self,
        url: Option<&str>,
        commitment: Option<Commitment>,
    ) -> anyhow::Result<GetLatestBlockhashOutput> {
        let body =
            GetLatestBlockhashInput::new(commitment.unwrap_or(self.default_commitment.clone()));
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
}
