use block_mesh_common::tracing::setup_tracing;
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{SolanaManager, SolanaManagerAuth};
use clap::Parser;
use futures_util::future::join_all;
use solana_sdk::pubkey::Pubkey;
use std::net::SocketAddr;
use std::str::FromStr;
use uuid::Uuid;

mod cli_args;
mod connection_listener;

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

    let nonce = Uuid::new_v4().to_string();
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair()).unwrap();
    let api_token = Pubkey::from_str("Ex4XxjYzNkzXWYj2dz95WzNXpbPRyaebryYi3Aqrrv82").unwrap();

    let solana_manager_header = SolanaManagerAuth::new(
        nonce,
        signed_message,
        solana_manager.get_pubkey(),
        api_token,
    );

    // let provider_node_address =
    //     get_provider_node_address(&cli_args.program_id, &provider_node_owner);
    //
    // let provider_node_account: ProviderNode = solana_manager
    //     .get_deserialized_account(&provider_node_address.0)
    //     .await
    //     .unwrap();

    let listener_task = tokio::spawn(connection_listener::listen_for_proxies_connecting(
        addr,
        solana_manager_header,
    ));

    let _ = join_all(vec![listener_task]).await;
    Ok(())
}
