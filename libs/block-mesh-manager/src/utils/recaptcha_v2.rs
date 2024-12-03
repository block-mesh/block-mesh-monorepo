use anyhow::anyhow;
use block_mesh_common::constants::{DeviceType, RECAPTCHA};
use block_mesh_common::interfaces::server_api::ReCaptchaV2;
use block_mesh_common::reqwest::http_client;

#[tracing::instrument(name = "recaptcha_v2", skip_all)]
pub async fn recaptcha_v2(recaptcha_v2: String, recaptcha_v2_secret: &str) -> anyhow::Result<()> {
    let client = http_client(DeviceType::AppServer);
    let response: serde_json::Value = client
        .post(RECAPTCHA)
        .form(&ReCaptchaV2 {
            secret: recaptcha_v2_secret.to_string(),
            response: recaptcha_v2,
        })
        .send()
        .await?
        .json()
        .await?;
    tracing::info!("recaptcha_v2 token check: {:?}", response);
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
