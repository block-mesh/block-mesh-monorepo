use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::utils::connectors::{pubkey, sign_message};
use anyhow::anyhow;
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, ConnectWalletRequest, ConnectWalletResponse, GetLatestInviteCodeRequest,
    GetLatestInviteCodeResponse, GetTokenResponse, LoginForm, RegisterForm, RegisterResponse,
};
use js_sys::Uint8Array;
use leptos::*;
use leptos_dom::tracing;
use uuid::Uuid;

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

#[tracing::instrument(name = "connect_wallet", err)]
pub async fn connect_wallet(
    origin: String,
    connect_wallet_request: ConnectWalletRequest,
) -> anyhow::Result<ConnectWalletResponse> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/connect_wallet", origin))
        .header("Content-Type", "application/json")
        .json(&connect_wallet_request)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

pub async fn connect_wallet_in_browser() {
    let msg = Uuid::new_v4().to_string();

    let key = pubkey().await;
    let sign = sign_message(&msg).await;
    let uint8_array = Uint8Array::new(&sign);
    let mut signature = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut signature[..]);

    let origin = window().origin();

    let pubkey = key.as_string().unwrap();

    let notifications = expect_context::<NotificationContext>();

    match connect_wallet(
        origin,
        ConnectWalletRequest {
            pubkey: pubkey.clone(),
            message: msg.to_string(),
            signature,
        },
    )
    .await
    {
        Ok(_) => {
            let auth = expect_context::<AuthContext>();
            auth.wallet_address.set(Some(pubkey));

            notifications.set_success("Connected successfully");
        }
        Err(_) => {
            notifications.set_error("Failed to connect");
        }
    }
}
