// use anchor_lang::Discriminator;
use block_mesh_common::cli::ProxyEndpointNodeOptions;
// use block_mesh_solana_client::helpers::{get_provider_node_address, sign_message};
// use block_mesh_solana_client::manager::{EndpointNodeToProviderNodeHeader, SolanaManager};
// use blockmesh_program::state::provider_node::ProviderNode;
use futures_util::future::join_all;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::str::FromStr;
// use std::sync::Arc;
// use uuid::Uuid;

mod connection_listener;
mod endpoint_headers;

#[tracing::instrument(name = "proxy_endpoint_main", ret, err)]
pub async fn proxy_endpoint_main(cli_args: &ProxyEndpointNodeOptions) -> anyhow::Result<ExitCode> {
    /*
    let mut solana_manager =
        SolanaManager::new(&cli_args.keypair_path, &cli_args.program_id).await?;
    let provider_node_account: ProviderNode = match cli_args.proxy_master_node_owner {
        Some(provider_node_owner) => {
            let provider_node_address =
                get_provider_node_address(&cli_args.program_id, &provider_node_owner);

            let provider_node_account: ProviderNode = solana_manager
                .get_deserialized_account(&provider_node_address.0)
                .await?;

            provider_node_account
        }
        None => {
            let provider_node_accounts = solana_manager
                .search_accounts(ProviderNode::discriminator(), vec![])
                .await?;
            tracing::info!(
                "Found {:?} Provider-Node accounts",
                provider_node_accounts.len()
            );
            if provider_node_accounts.is_empty() {
                tracing::error!("No provider node found");
                exit(1);
            } else if provider_node_accounts.len() > 1 {
                tracing::info!(
                    "Multiple provider nodes found, taking the first one - {:?}",
                    provider_node_accounts[0]
                );
            }
            let provider_node_account: ProviderNode = solana_manager
                .get_deserialized_account(&provider_node_accounts[0].0)
                .await?;
            provider_node_account
        }
    };
    let proxy_url = match cli_args.proxy_override.clone() {
        Some(proxy_override) => proxy_override,
        None => {
            format!(
                "{}.{}.{}.{}:{}",
                provider_node_account.ipv4[0],
                provider_node_account.ipv4[1],
                provider_node_account.ipv4[2],
                provider_node_account.ipv4[3],
                provider_node_account.proxy_port
            )
        }
    };
     */
    let proxy_url = "0.0.0.0".to_string();
    tracing::info!("Proxy URL: {}", proxy_url);
    let addr = SocketAddr::from_str(proxy_url.as_str()).expect("Failed to parse address");
    // solana_manager.create_endpoint_account_if_needed().await?;
    // let solana_manager = Arc::new(solana_manager);
    // let nonce = Uuid::new_v4().to_string();
    // let signature = sign_message(&nonce, &solana_manager.get_keypair())?;
    // let auth_header: EndpointNodeToProviderNodeHeader = EndpointNodeToProviderNodeHeader {
    //     nonce,
    //     signature,
    //     pubkey: solana_manager.get_pubkey(),
    // };
    let listener_task = tokio::spawn(connection_listener::listen_for_proxies_connecting(
        addr,
        // auth_header,
        // solana_manager,
    ));

    let _ = join_all(vec![listener_task]).await;
    Ok(ExitCode::SUCCESS)
}
