use anyhow::anyhow;
use block_mesh_common::constants::{DeviceType, CF_TURNSTILE};
use block_mesh_common::interfaces::server_api::TurnStile;
use block_mesh_common::reqwest::http_client;

#[tracing::instrument(name = "check_cf_token", skip_all)]
pub async fn check_cf_token(cftoken: String, cf_secret_key: &str) -> anyhow::Result<()> {
    let client = http_client(DeviceType::AppServer);
    let response: serde_json::Value = client
        .post(CF_TURNSTILE)
        .form(&TurnStile {
            secret: cf_secret_key.to_string(),
            response: cftoken,
        })
        .send()
        .await?
        .json()
        .await?;
    tracing::info!("CF token check: {:?}", response);
    match response.get("success") {
        Some(s) => match s.as_bool() {
            Some(true) => Ok(()),
            _ => Err(anyhow!("Failed to prove you are human")),
        },
        None => Err(anyhow!(
            "Error running CFTOKEN Failed to prove you are human"
        )),
    }
}
