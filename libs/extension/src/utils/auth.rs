use anyhow::anyhow;

use block_mesh_common::interface::{CheckTokenRequest, GetTokenResponse, LoginForm, RegisterForm};

pub async fn check_token(
    blockmesh_url: &str,
    credentials: &CheckTokenRequest,
) -> anyhow::Result<GetTokenResponse> {
    let url = format!("{}/api/check_token", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(credentials)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

pub async fn register(blockmesh_url: &str, credentials: &RegisterForm) -> anyhow::Result<()> {
    let url = format!("{}/register", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client.post(&url).form(credentials).send().await?;
    if response.status().is_success() || response.status().is_redirection() {
        Ok(())
    } else {
        Err(anyhow!("Failed to register"))
    }
}

pub async fn login(
    blockmesh_url: &str,
    credentials: &LoginForm,
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
