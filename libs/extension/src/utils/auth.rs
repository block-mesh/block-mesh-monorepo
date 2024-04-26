use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginCreds {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterCreds {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

use block_mesh_common::interface::GetTokenResponse;

pub async fn _register(blockmesh_url: &str, credentials: &RegisterCreds) -> anyhow::Result<()> {
    let url = format!("{}/register_post", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client.post(&url).json(credentials).send().await?;
    if response.status().is_success() || response.status().is_redirection() {
        Ok(())
    } else {
        Err(anyhow!("Failed to register"))
    }
}

pub async fn login(
    blockmesh_url: &str,
    credentials: &LoginCreds,
) -> anyhow::Result<GetTokenResponse> {
    let url = format!("{}/api/get_token", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&credentials)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
