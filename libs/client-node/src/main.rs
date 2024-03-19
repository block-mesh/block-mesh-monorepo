mod cli_args;

use crate::cli_args::ClientNodeCliArgs;
use block_mesh_solana_client::helpers::{get_provider_node_address, sign_message};
use block_mesh_solana_client::manager::SolanaManager;
use blockmesh_program::state::provider_node::ProviderNode;
use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_client::client_error::reqwest;
use solana_client::client_error::reqwest::Proxy;
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
    let provider_node_owner = client_node_cli_args.provider_node_owner;
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

    let provider_node_address =
        get_provider_node_address(&client_node_cli_args.program_id, &provider_node_owner);

    let provider_node_account: ProviderNode = solana_manager
        .get_deserialized_account(&provider_node_address.0)
        .await
        .unwrap();

    let proxy_url = match client_node_cli_args.proxy_override {
        Some(proxy_override) => proxy_override,
        None => {
            format!(
                "http://{}.{}.{}.{}:{}",
                provider_node_account.ipv4[0],
                provider_node_account.ipv4[1],
                provider_node_account.ipv4[2],
                provider_node_account.ipv4[3],
                provider_node_account.port
            )
        }
    };
    tracing::info!("Proxy URL: {}", proxy_url);
    let nonce = "im a nonce";
    let signed_message = sign_message(nonce, &solana_manager.get_keypair()).unwrap();

    let proxy = get_proxy(&proxy_url, nonce, &signed_message).await.unwrap();

    let local_address = IpAddr::from_str("0.0.0.0").unwrap();

    let client = reqwest::Client::builder()
        .local_address(local_address)
        .proxy(proxy)
        .build()
        .unwrap();
    match client_node_cli_args.response_type {
        cli_args::ResponseType::Json => {
            let response: serde_json::Value = client
                .get(&client_node_cli_args.target)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            println!("FINAL RESPONSE => {:?}", response);
        }
        cli_args::ResponseType::Text => {
            let response: String = client
                .get(&client_node_cli_args.target)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("FINAL RESPONSE => {:?}", response);
        }
    }
}
