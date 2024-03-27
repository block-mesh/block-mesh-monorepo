use crate::api_token::create_api_token_instruction::create_api_token_instruction;
use crate::client::create_client::create_client_instruction;
use crate::client::update_latest_client_report::update_latest_client_report_instruction;
use crate::helpers::{
    build_txn_and_send_and_confirm, get_account, get_api_token_address, get_client,
    get_client_address, get_provider_node_address, CloneableKeypair,
};
use crate::provider_node::create_provider_node::create_provider_node_instruction;
use crate::provider_node::update_provider_node::update_provider_node_instruction;
use anchor_lang::AccountDeserialize;
use anyhow::anyhow;
use secret::Secret;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::fs::try_exists;

#[derive(Clone)]
pub struct SolanaManager {
    keypair: Arc<Secret<CloneableKeypair>>,
    program_id: Pubkey,
    provider_node: Option<Pubkey>,
    client: Option<Pubkey>,
    api_token: Option<Pubkey>,
    rpc_client: Arc<RpcClient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaManagerAuth {
    pub nonce: String,
    pub signed_message: String,
    pub pubkey: Pubkey,
    pub api_token: Pubkey,
}

impl SolanaManagerAuth {
    pub fn new(nonce: String, signed_message: String, pubkey: Pubkey, api_token: Pubkey) -> Self {
        Self {
            nonce,
            signed_message,
            pubkey,
            api_token,
        }
    }
}

impl SolanaManager {
    pub fn get_keypair(&self) -> Keypair {
        self.keypair.clone().expose_secret().keypair()
    }

    pub fn get_pubkey(&self) -> Pubkey {
        self.keypair.clone().expose_secret().pubkey()
    }

    pub fn get_api_token(&self) -> Pubkey {
        self.api_token.unwrap()
    }

    #[tracing::instrument(name = "SolanaManager::new")]
    pub async fn new(keypair_path: &str, program_id: &Pubkey) -> anyhow::Result<Self> {
        try_exists(&keypair_path).await?;
        let keypair = solana_sdk::signature::read_keypair_file(keypair_path)
            .map_err(|e| anyhow!("Error reading keypair file: {}", e))?;
        let keypair = Secret::from(CloneableKeypair::new(keypair));
        tracing::info!("Provider Node pubkey {}", keypair.as_ref().pubkey());
        Ok(Self {
            keypair: Arc::new(keypair),
            program_id: *program_id,
            provider_node: None,
            rpc_client: Arc::new(get_client()),
            client: None,
            api_token: None,
        })
    }

    #[tracing::instrument(name = "SolanaManager::deserialize", ret, err)]
    pub fn deserialize<T>(account: Account) -> anyhow::Result<T>
    where
        T: std::fmt::Debug + AccountDeserialize,
    {
        let result: T = T::try_deserialize(&mut account.data.as_slice())?;
        Ok(result)
    }

    #[tracing::instrument(name = "get_deserialized_account", skip(self, address), ret, err)]
    pub async fn get_deserialized_account<T>(&self, address: &Pubkey) -> anyhow::Result<T>
    where
        T: std::fmt::Debug + AccountDeserialize,
    {
        let account = get_account(&self.rpc_client, address).await?;
        match account {
            Some(account) => {
                let result: T = T::try_deserialize(&mut account.data.as_slice())?;
                Ok(result)
            }
            None => Err(anyhow!("Account not found")),
        }
    }

    #[tracing::instrument(name = "create_api_token_if_needed", skip(self), ret, err)]
    pub async fn create_api_token_if_needed(
        &mut self,
        provider_node_owner: &Pubkey,
    ) -> anyhow::Result<()> {
        let provider_node_address =
            get_provider_node_address(&self.program_id, provider_node_owner);

        let api_token_address =
            get_api_token_address(&self.program_id, &self.get_pubkey(), provider_node_owner);

        let account = get_account(&self.rpc_client, &api_token_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_api_token_if_needed::Error getting api_token account: {}",
                    e.to_string()
                )
            })?;

        self.api_token = Some(api_token_address.0);

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
                    self.get_pubkey(),
                    self.client.unwrap(),
                    api_token_address.0,
                    provider_node_address.0,
                );
                let signature = build_txn_and_send_and_confirm(
                    &self.rpc_client,
                    vec![instruction],
                    &self.get_pubkey(),
                    &self.get_keypair(),
                )
                .await?;
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
        let client_address = get_client_address(&self.program_id, &self.get_pubkey());
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
                let instruction =
                    create_client_instruction(self.program_id, self.get_pubkey(), client_address.0);
                let signature = build_txn_and_send_and_confirm(
                    &self.rpc_client,
                    vec![instruction],
                    &self.get_pubkey(),
                    &self.get_keypair(),
                )
                .await?;
                tracing::info!(
                    "create_client_account_if_needed::Client account created: {}",
                    signature
                );
                Ok(())
            }
        }
    }

    #[tracing::instrument(
        name = "create_or_update_provider_node_if_needed",
        skip(self),
        ret,
        err
    )]
    pub async fn create_or_update_provider_node_if_needed(
        &mut self,
        ip_addr: Ipv4Addr,
        proxy_port: u16,
        client_port: u16,
    ) -> anyhow::Result<()> {
        let provider_node_address = get_provider_node_address(&self.program_id, &self.get_pubkey());
        self.provider_node = Some(provider_node_address.0);
        let account = get_account(&self.rpc_client, &provider_node_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_or_update_provider_node_if_needed::Error getting provider node account: {}",
                    e.to_string()
                )
            })?;
        let instruction: Option<Instruction> = match account {
            Some(_account) => {
                tracing::info!(
                    "create_or_update_provider_node_if_needed::Provider need to be updated: {:?}",
                    &provider_node_address.0.to_string()
                );
                let instruction = update_provider_node_instruction(
                    self.program_id,
                    ip_addr.octets(),
                    proxy_port,
                    client_port,
                    100,
                    self.get_pubkey(),
                    provider_node_address.0,
                );
                Some(instruction)
            }
            None => {
                let instruction = create_provider_node_instruction(
                    self.program_id,
                    ip_addr.octets(),
                    proxy_port,
                    client_port,
                    100,
                    self.get_pubkey(),
                    provider_node_address.0,
                );
                Some(instruction)
            }
        };
        if let Some(instruction) = instruction {
            let signature = build_txn_and_send_and_confirm(
                &self.rpc_client,
                vec![instruction],
                &self.get_pubkey(),
                &self.get_keypair(),
            )
            .await?;
            tracing::info!(
                "create_or_update_provider_node_if_needed::Transaction sent: {}",
                signature
            );
        }
        Ok(())
    }

    #[tracing::instrument(name = "update_latest_client_report", skip(self), ret, err)]
    pub async fn update_latest_client_report(
        &self,
        provider_node_owner: &Pubkey,
        latest_client_report: u64,
    ) -> anyhow::Result<()> {
        let provider_node_address =
            get_provider_node_address(&self.program_id, provider_node_owner);
        let instruction = update_latest_client_report_instruction(
            self.program_id,
            self.get_pubkey(),
            self.client.unwrap(),
            self.api_token.unwrap(),
            provider_node_address.0,
            latest_client_report,
        );
        let signature = build_txn_and_send_and_confirm(
            &self.rpc_client,
            vec![instruction],
            &self.get_pubkey(),
            &self.get_keypair(),
        )
        .await?;
        tracing::info!(
            "update_latest_client_report::Transaction sent: {}",
            signature
        );
        Ok(())
    }
}
