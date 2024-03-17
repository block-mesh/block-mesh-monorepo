mod cli_args;

use crate::cli_args::ClientNodeCliArgs;
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::SolanaManager;
use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_client::client_error::reqwest;
use solana_client::client_error::reqwest::Proxy;
use solana_sdk::pubkey::Pubkey;
use std::net::IpAddr;
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
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();

    let client_node_cli_args = ClientNodeCliArgs::parse();
    let provider_node_owner = Pubkey::from_str(&client_node_cli_args.provider_node_owner).unwrap();
    let mut solana_manager = SolanaManager::new(
        &client_node_cli_args.keypair_path,
        &client_node_cli_args.program_id,
    )
    .await
    .unwrap();
    solana_manager
        .create_client_account_if_needed()
        .await
        .unwrap();
    solana_manager
        .create_api_token_if_needed(&provider_node_owner)
        .await
        .unwrap();

    let nonce = "im a nonce";
    let signed_message = sign_message(nonce, &solana_manager.get_keypair()).unwrap();
    let proxy = get_proxy("http://127.0.0.1:3000", nonce, &signed_message)
        .await
        .unwrap();

    let local_address = IpAddr::from_str("0.0.0.0").unwrap();

    let client = reqwest::Client::builder()
        .local_address(local_address)
        .proxy(proxy)
        .build()
        .unwrap();
    let response: serde_json::Value = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("FINAL RESPONSE => {:?}", response);
}
