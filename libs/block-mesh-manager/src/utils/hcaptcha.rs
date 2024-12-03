use anyhow::anyhow;
use block_mesh_common::constants::{DeviceType, HCAPTCHA};
use block_mesh_common::interfaces::server_api::HCaptcha;
use block_mesh_common::reqwest::http_client;

#[tracing::instrument(name = "hcaptcha", skip_all)]
pub async fn hcaptcha(hcaptcha: String, hcaptcha_secret: &str) -> anyhow::Result<()> {
    let client = http_client(DeviceType::AppServer);
    let response: serde_json::Value = client
        .post(HCAPTCHA)
        .form(&HCaptcha {
            secret: hcaptcha_secret.to_string(),
            response: hcaptcha.to_string(),
        })
        .send()
        .await?
        .json()
        .await?;
    tracing::info!("hcaptcha token check: {:?}", response);
    match response.get("success") {
        Some(s) => match s.as_bool() {
            Some(true) => Ok(()),
            _ => Err(anyhow!("Failed to prove you are human")),
        },
        None => Err(anyhow!(
            "Error running ReCaptcha V2 Failed to prove you are human"
        )),
    }
}
