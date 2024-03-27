use block_mesh_common::tracing::setup_tracing;
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{EndpointNodeToProviderNodeHeader, SolanaManager};
use clap::Parser;
use futures_util::future::join_all;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

mod cli_args;
mod connection_listener;
mod endpoint_headers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();
    let cli_args = cli_args::CliArgs::parse();
    let addr = SocketAddr::from_str(format!("{}:{}", cli_args.ip, cli_args.port).as_str())
        .expect("Failed to parse address");

    // let provider_node_owner = cli_args.provider_node_owner;
    let mut solana_manager = SolanaManager::new(&cli_args.keypair_path, &cli_args.program_id)
        .await
        .unwrap();
    solana_manager
        .create_endpoint_account_if_needed()
        .await
        .unwrap();

    let solana_manager = Arc::new(solana_manager);
    let nonce = Uuid::new_v4().to_string();
    let signature = sign_message(&nonce, &solana_manager.get_keypair()).unwrap();

    let auth_header: EndpointNodeToProviderNodeHeader = EndpointNodeToProviderNodeHeader {
        nonce,
        signature,
        pubkey: solana_manager.get_pubkey(),
    };

    // let provider_node_address =
    //     get_provider_node_address(&cli_args.program_id, &provider_node_owner);
    //
    // let provider_node_account: ProviderNode = solana_manager
    //     .get_deserialized_account(&provider_node_address.0)
    //     .await
    //     .unwrap();

    let listener_task = tokio::spawn(connection_listener::listen_for_proxies_connecting(
        addr,
        auth_header,
        solana_manager,
    ));

    let _ = join_all(vec![listener_task]).await;
    Ok(())
}
