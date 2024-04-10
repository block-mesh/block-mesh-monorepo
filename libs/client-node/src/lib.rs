mod management;
mod modes;

use crate::modes::cli::cli_mode;
use crate::modes::proxy_mode::proxy_mode;
use anchor_lang::Discriminator;
use block_mesh_common::cli::{ClientNodeMode, ClientNodeOptions};
use block_mesh_common::tracing::setup_tracing;
use block_mesh_solana_client::helpers::get_provider_node_address;
use block_mesh_solana_client::manager::{FullRouteHeader, SolanaManager};
use blockmesh_program::state::provider_node::ProviderNode;
use solana_client::client_error::reqwest::Proxy;
use std::process::{exit, ExitCode};
use std::sync::Arc;

#[tracing::instrument(name = "get_proxy", ret, err)]
pub async fn get_proxy(
    proxy_url: &str,
    solana_manager_header: &FullRouteHeader,
) -> anyhow::Result<Proxy> {
    let proxy = Proxy::all(proxy_url)?;
    let json = serde_json::to_string(solana_manager_header)?;
    let proxy = proxy.custom_http_auth(json.parse()?); // Proxy-Authorization
    Ok(proxy)
}

#[tracing::instrument(name = "client_node_main", ret, err)]
pub async fn client_node_main(client_node_cli_args: ClientNodeOptions) -> anyhow::Result<ExitCode> {
    setup_tracing();
    let mut solana_manager = SolanaManager::new(
        &client_node_cli_args.keypair_path,
        &client_node_cli_args.program_id,
    )
    .await?;
    solana_manager.create_client_account_if_needed().await?;
    let provider_node_account: ProviderNode = match client_node_cli_args.proxy_master_node_owner {
        Some(provider_node_owner) => {
            let provider_node_address =
                get_provider_node_address(&client_node_cli_args.program_id, &provider_node_owner);
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

    solana_manager
        .create_api_token_if_needed(&provider_node_account.owner)
        .await?;

    let proxy_url = match client_node_cli_args.proxy_override {
        Some(ref proxy_override) => proxy_override.to_string(),
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
    let solana_manager = Arc::new(solana_manager);
    // register_token(
    //     &format!("http://{}/register", proxy_url),
    //     &solana_manager_header,
    // )
    // .await
    // ?;

    match &client_node_cli_args.mode {
        ClientNodeMode::Cli => {
            tracing::info!("Starting in CLI mode");
            cli_mode(solana_manager, &proxy_url, client_node_cli_args).await?;
        }
        ClientNodeMode::Proxy => {
            tracing::info!("Starting in proxy mode");
            proxy_mode(
                solana_manager,
                Arc::new(proxy_url.to_string()),
                client_node_cli_args,
            )
            .await?;
        }
    };
    Ok(ExitCode::SUCCESS)
}
