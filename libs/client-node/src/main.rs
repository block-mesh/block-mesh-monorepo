mod cli_args;
mod management;

use crate::cli_args::ClientNodeCliArgs;
use block_mesh_common::tracing::setup_tracing;
use block_mesh_solana_client::helpers::{get_provider_node_address, sign_message};
use block_mesh_solana_client::manager::{FullRouteHeader, SolanaManager};
use blockmesh_program::state::provider_node::ProviderNode;
use clap::Parser;
use solana_client::client_error::reqwest;
use solana_client::client_error::reqwest::Proxy;
use std::net::IpAddr;
use std::str::FromStr;
use uuid::Uuid;

pub async fn get_proxy(
    proxy_url: &str,
    solana_manager_header: &FullRouteHeader,
) -> anyhow::Result<Proxy> {
    let proxy = Proxy::all(proxy_url)?;
    let json = serde_json::to_string(solana_manager_header)?;
    let proxy = proxy.custom_http_auth(json.parse().unwrap()); // Proxy-Authorization
    Ok(proxy)
}

#[tokio::main]
async fn main() {
    setup_tracing();
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
                provider_node_account.client_port
            )
        }
    };
    tracing::info!("Proxy URL: {}", proxy_url);
    let nonce = Uuid::new_v4().to_string();
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair()).unwrap();

    let solana_manager_header = FullRouteHeader::new(
        nonce,
        signed_message,
        solana_manager.get_pubkey(),
        solana_manager.get_api_token(),
        "client-node".to_string(),
    );
    // register_token(
    //     &format!("http://{}/register", proxy_url),
    //     &solana_manager_header,
    // )
    // .await
    // .unwrap();
    let proxy = get_proxy(&proxy_url, &solana_manager_header).await.unwrap();
    let local_address = IpAddr::from_str("0.0.0.0").unwrap();
    let client = reqwest::Client::builder()
        .local_address(local_address)
        .proxy(proxy)
        .build()
        .unwrap();

    let response: reqwest::Response = client
        .get(&client_node_cli_args.target)
        .send()
        .await
        .unwrap();
    tracing::info!("RESPONSE HEADERS => {:?}", response.headers());
    let content_type = match response.headers().get("content-type") {
        None => client_node_cli_args.response_type,
        Some(content_type) => {
            if content_type.to_str().unwrap().contains("application/json") {
                cli_args::ResponseType::Json
            } else {
                cli_args::ResponseType::Text
            }
        }
    };
    match content_type {
        cli_args::ResponseType::Json => {
            let response: serde_json::Value = response.json().await.unwrap();
            tracing::info!("FINAL RESPONSE => {:?}", response);
        }
        cli_args::ResponseType::Text => {
            let response: String = response.text().await.unwrap();
            tracing::info!("FINAL RESPONSE => {:?}", response);
        }
    }
}
