use anyhow::anyhow;
use block_mesh_common::constants::BLOCK_MESH_APP_SERVER;
use block_mesh_common::interfaces::server_api::{GetTokenResponse, LoginForm};
use uuid::Uuid;

pub async fn login(login_form: LoginForm) -> anyhow::Result<Uuid> {
    let url = format!("{}/api/get_token", BLOCK_MESH_APP_SERVER);
    let client = reqwest::Client::new();
    let response: GetTokenResponse = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&login_form)
        .send()
        .await
        .map_err(|e| anyhow!(e.to_string()))?
        .json()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    match response.api_token {
        Some(api_token) => Ok(api_token),
        None => Err(anyhow!("missing api_token")),
    }
}
