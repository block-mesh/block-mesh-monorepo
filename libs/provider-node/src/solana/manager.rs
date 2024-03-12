use crate::solana::helpers::{
    create_transaction, get_account, get_client, get_provider_node_address, get_recent_blockhash,
    send_and_confirm_transaction,
};
use crate::solana::provider_node::create_provider_node::create_provider_node_instruction;
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::try_exists;

#[derive(Clone)]
pub struct SolanaManager {
    keypair: Arc<Keypair>,
    program_id: Pubkey,
    provider_account: Option<Pubkey>,
    rpc_client: Arc<RpcClient>,
}

impl SolanaManager {
    #[tracing::instrument(name = "SolanaManager::new")]
    pub async fn new() -> anyhow::Result<Self> {
        let keypair_envar = std::env::var("PROVIDER_NODE_KEYPAIR")?;
        try_exists(&keypair_envar).await?;
        let keypair = solana_sdk::signature::read_keypair_file(&keypair_envar)
            .map_err(|e| anyhow!("Error reading keypair file: {}", e))?;

        tracing::info!("Provider Node pubkey {}", keypair.pubkey());
        Ok(Self {
            keypair: Arc::new(keypair),
            program_id: Pubkey::from_str("CfaL9sdaEK49r4WLAtVh2vVgAZuv2eKbb6jSB5jDCMSF")?,
            provider_account: None,
            rpc_client: Arc::new(get_client()),
        })
    }

    #[tracing::instrument(name = "create_provider_account_if_needed", skip(self), ret, err)]
    pub async fn create_provider_account_if_needed(&mut self) -> anyhow::Result<()> {
        let provider_node_address =
            get_provider_node_address(&self.program_id, &self.keypair.pubkey());
        self.provider_account = Some(provider_node_address.0);
        let account = get_account(&self.rpc_client, &provider_node_address.0)
            .await
            .map_err(|e| anyhow!("Error getting provider node account: {}", e.to_string()))?;
        match account {
            Some(_) => {
                tracing::info!(
                    "Provider node account already exists: {:?}",
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
                    .map_err(|e| anyhow!("Error sending transaction: {}", e.to_string()))?;
                tracing::info!("Provider node account created: {}", signature);
                Ok(())
            }
        }
    }
}
