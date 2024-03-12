use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{SolanaManager, SolanaManagerMode};
use serde::{Deserialize, Serialize};
use solana_client::client_error::reqwest;
use solana_client::client_error::reqwest::Proxy;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Deserialize, Serialize)]
struct ProxyAuthentication {
    nonce: String,
    signed_message: String,
}

pub async fn get_proxy(
    proxy_url: &str,
    nonce: &str,
    signed_message: &str,
) -> anyhow::Result<Proxy> {
    let proxy = Proxy::all(proxy_url)?;
    let json = serde_json::to_string(&ProxyAuthentication {
        nonce: nonce.to_string(),
        signed_message: signed_message.to_string(),
    })?;
    let proxy = proxy.custom_http_auth(json.parse().unwrap()); // Proxy-Authorization
    Ok(proxy)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let provider_node_owner =
        Pubkey::from_str("CERqu7FToQX6c1VGhDojaaFTcMX2H8vBPBbwmPnKfQdY").unwrap();
    let mut solana_manager = SolanaManager::new(SolanaManagerMode::Client).await.unwrap();
    solana_manager
        .create_client_account_if_needed()
        .await
        .unwrap();
    solana_manager
        .create_api_token_if_needed(&provider_node_owner)
        .await
        .unwrap();

    let nonce = "im a nonce";
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair()).unwrap();
    let proxy = get_proxy("http://127.0.0.1:3000", nonce, &signed_message)
        .await
        .unwrap();
    let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let response = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await
        .unwrap();
    println!("{:?}", response);
}
