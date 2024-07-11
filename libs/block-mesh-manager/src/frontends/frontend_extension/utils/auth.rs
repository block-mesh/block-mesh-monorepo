use anyhow::anyhow;
use leptos::leptos_dom;
use leptos_dom::tracing;

use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, GetLatestInviteCodeRequest, GetLatestInviteCodeResponse, GetTokenResponse,
    LoginForm, RegisterForm, RegisterResponse,
};

#[tracing::instrument(name = "check_token", skip(credentials), err)]
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

#[tracing::instrument(name = "register", skip(credentials), err)]
pub async fn register(blockmesh_url: &str, credentials: &RegisterForm) -> anyhow::Result<()> {
    let url = format!("{}/register_api", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client.post(&url).form(credentials).send().await?;
    let response: RegisterResponse = response.json().await?;
    if response.status_code == 200 {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to register - {}",
            response.error.unwrap_or_default()
        ))
    }
}

#[tracing::instrument(name = "login", skip(credentials), err)]
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

#[tracing::instrument(name = "get_latest_invite_code", skip(credentials), err)]
pub async fn get_latest_invite_code(
    blockmesh_url: &str,
    credentials: &GetLatestInviteCodeRequest,
) -> anyhow::Result<GetLatestInviteCodeResponse> {
    let url = format!("{}/api/get_latest_invite_code", blockmesh_url);
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
