use block_mesh_common::constants::BLOCKMESH_CAPTCHA;
use block_mesh_common::interfaces::server_api::CaptchaResp;

#[allow(dead_code)]
pub async fn captcha_check() -> anyhow::Result<CaptchaResp> {
    let client = reqwest::Client::new();
    let r = client
        .post(BLOCKMESH_CAPTCHA)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    if r.status().is_success() {
        let json: CaptchaResp = r.json().await?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!("Unexpected status: {:?}", r.status()))
    }
}
