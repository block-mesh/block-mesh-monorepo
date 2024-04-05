use crate::cli_args::ClientNodeCliArgs;
use crate::{cli_args, get_proxy};
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{FullRouteHeader, SolanaManager};
use solana_client::client_error::reqwest;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "cli_mode", skip(solana_manager), ret, err)]
pub async fn cli_mode(
    solana_manager: Arc<SolanaManager>,
    proxy_url: &str,
    client_node_cli_args: ClientNodeCliArgs,
) -> anyhow::Result<()> {
    let nonce = Uuid::new_v4().to_string();
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair())?;
    let solana_manager_header = FullRouteHeader::new(
        nonce,
        signed_message,
        solana_manager.get_pubkey(),
        solana_manager.get_api_token(),
        "client-node".to_string(),
    );
    let proxy = get_proxy(proxy_url, &solana_manager_header).await?;
    let local_address = IpAddr::from_str("0.0.0.0")?;
    let client = reqwest::Client::builder()
        .local_address(local_address)
        .proxy(proxy)
        .build()?;

    let response: reqwest::Response = client.get(&client_node_cli_args.target).send().await?;
    tracing::info!("RESPONSE HEADERS => {:?}", response.headers());
    let content_type = match response.headers().get("content-type") {
        None => cli_args::ResponseType::Text,
        Some(content_type) => {
            if content_type.to_str()?.contains("application/json") {
                cli_args::ResponseType::Json
            } else {
                cli_args::ResponseType::Text
            }
        }
    };
    match content_type {
        cli_args::ResponseType::Json => {
            let response: serde_json::Value = response.json().await?;
            let pretty_json = serde_json::to_string_pretty(&response)?;
            tracing::info!("FINAL RESPONSE: {:?}", pretty_json);
            println!("\n\n{}\n\n", pretty_json);
        }
        cli_args::ResponseType::Text => {
            let response: String = response.text().await?;
            tracing::info!("FINAL RESPONSE: {:?}", response);
            println!("\n\n{}\n\n", response);
        }
    }
    Ok(())
}
