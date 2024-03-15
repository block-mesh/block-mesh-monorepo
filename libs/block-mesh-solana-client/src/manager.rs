use crate::api_token::create_api_token_instruction::create_api_token_instruction;
use crate::client::create_client::create_client_instruction;
use crate::helpers::{
    create_transaction, get_account, get_api_token_address, get_client, get_client_address,
    get_provider_node_address, get_recent_blockhash, send_and_confirm_transaction,
};
use crate::provider_node::create_provider_node::create_provider_node_instruction;
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;
use tokio::fs::try_exists;

#[derive(Clone)]
pub struct SolanaManager {
    keypair: Arc<Keypair>,
    program_id: Pubkey,
    provider_account: Option<Pubkey>,
    client: Option<Pubkey>,
    rpc_client: Arc<RpcClient>,
}

impl SolanaManager {
    pub fn get_keypair(&self) -> Arc<Keypair> {
        self.keypair.clone()
    }

    #[tracing::instrument(name = "SolanaManager::new")]
    pub async fn new(keypair_path: &str, program_id: &Pubkey) -> anyhow::Result<Self> {
        try_exists(&keypair_path).await?;
        let keypair = solana_sdk::signature::read_keypair_file(keypair_path)
            .map_err(|e| anyhow!("Error reading keypair file: {}", e))?;

        tracing::info!("Provider Node pubkey {}", keypair.pubkey());
        Ok(Self {
            keypair: Arc::new(keypair),
            program_id: *program_id,
            provider_account: None,
            rpc_client: Arc::new(get_client()),
            client: None,
        })
    }

    #[tracing::instrument(name = "create_api_token_if_needed", skip(self), ret, err)]
    pub async fn create_api_token_if_needed(
        &mut self,
        provider_node_owner: &Pubkey,
    ) -> anyhow::Result<()> {
        let provider_node_address =
            get_provider_node_address(&self.program_id, provider_node_owner);
        let api_token_address = get_api_token_address(
            &self.program_id,
            &self.keypair.pubkey(),
            provider_node_owner,
        );

        let account = get_account(&self.rpc_client, &api_token_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_api_token_if_needed::Error getting api_token account: {}",
                    e.to_string()
                )
            })?;

        match account {
            Some(_) => {
                tracing::info!(
                    "create_api_token_if_needed::ApiToken account already exists: {:?}",
                    &api_token_address.0.to_string()
                );
                Ok(())
            }
            None => {
                let instruction = create_api_token_instruction(
                    self.program_id,
                    self.keypair.pubkey(),
                    self.client.unwrap(),
                    api_token_address.0,
                    provider_node_address.0,
                );

                let latest_blockhash = get_recent_blockhash(&self.rpc_client).await?;
                let txn = create_transaction(
                    vec![instruction],
                    &self.keypair.pubkey(),
                    &self.keypair,
                    latest_blockhash,
                )?;
                let signature = send_and_confirm_transaction(&self.rpc_client, &txn)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "create_api_token_if_needed::Error sending transaction: {}",
                            e.to_string()
                        )
                    })?;
                tracing::info!(
                    "create_api_token_if_needed::ApiToken account created: {}",
                    signature
                );
                Ok(())
            }
        }
    }

    #[tracing::instrument(name = "create_client_account_if_needed", skip(self), ret, err)]
    pub async fn create_client_account_if_needed(&mut self) -> anyhow::Result<()> {
        let client_address = get_client_address(&self.program_id, &self.keypair.pubkey());
        self.client = Some(client_address.0);
        let account = get_account(&self.rpc_client, &client_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_client_account_if_needed::Error getting client account: {}",
                    e.to_string()
                )
            })?;
        match account {
            Some(_) => {
                tracing::info!(
                    "create_client_account_if_needed::Provider node account already exists: {:?}",
                    &client_address.0.to_string()
                );
                Ok(())
            }
            None => {
                let instruction = create_client_instruction(
                    self.program_id,
                    self.keypair.pubkey(),
                    client_address.0,
                );
                let latest_blockhash = get_recent_blockhash(&self.rpc_client).await?;
                let txn = create_transaction(
                    vec![instruction],
                    &self.keypair.pubkey(),
                    &self.keypair,
                    latest_blockhash,
                )?;
                let signature = send_and_confirm_transaction(&self.rpc_client, &txn)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "create_client_account_if_needed::Error sending transaction: {}",
                            e.to_string()
                        )
                    })?;
                tracing::info!(
                    "create_client_account_if_needed::Client account created: {}",
                    signature
                );
                Ok(())
            }
        }
    }

    #[tracing::instrument(name = "create_provider_account_if_needed", skip(self), ret, err)]
    pub async fn create_provider_account_if_needed(&mut self) -> anyhow::Result<()> {
        let provider_node_address =
            get_provider_node_address(&self.program_id, &self.keypair.pubkey());
        self.provider_account = Some(provider_node_address.0);
        let account = get_account(&self.rpc_client, &provider_node_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_provider_account_if_needed::Error getting provider node account: {}",
                    e.to_string()
                )
            })?;
        match account {
            Some(_) => {
                tracing::info!(
                    "create_provider_account_if_needed::Provider node account already exists: {:?}",
                    &provider_node_address.0.to_string()
                );
                Ok(())
            }
            None => {
                let instruction = create_provider_node_instruction(
                    self.program_id,
                    [127, 0, 0, 1],
                    3000,
                    100,
                    self.keypair.pubkey(),
                    provider_node_address.0,
                );
                let latest_blockhash = get_recent_blockhash(&self.rpc_client).await?;
                let txn = create_transaction(
                    vec![instruction],
                    &self.keypair.pubkey(),
                    &self.keypair,
                    latest_blockhash,
                )?;
                let signature = send_and_confirm_transaction(&self.rpc_client, &txn)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "create_provider_account_if_needed::Error sending transaction: {}",
                            e.to_string()
                        )
                    })?;
                tracing::info!(
                    "create_provider_account_if_needed::Provider node account created: {}",
                    signature
                );
                Ok(())
            }
        }
    }
}
