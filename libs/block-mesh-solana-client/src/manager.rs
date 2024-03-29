use crate::api_token::create_api_token_instruction::create_api_token_instruction;
use crate::client::create_client::create_client_instruction;
use crate::client::update_latest_client_report::update_latest_client_report_instruction;
use crate::endpoint::create_endpoint_node::create_endpoint_node;
use crate::helpers::{
    build_txn_and_send_and_confirm, get_account, get_api_token_address, get_client,
    get_client_address, get_endpoint_address, get_provider_node_address, CloneableKeypair,
};
use crate::provider_node::create_provider_node::create_provider_node_instruction;
use crate::provider_node::update_provider_node::update_provider_node_instruction;
use anchor_lang::AccountDeserialize;
use anyhow::anyhow;
use secret::Secret;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use spl_memo::build_memo;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::try_exists;

#[derive(Clone)]
pub struct SolanaManager {
    keypair: Arc<Secret<CloneableKeypair>>,
    program_id: Pubkey,
    provider_node: Option<Pubkey>,
    endpoint_node: Option<Pubkey>,
    client: Option<Pubkey>,
    api_token: Option<Pubkey>,
    rpc_client: Arc<RpcClient>,
}

fn serialize_pubkey_as_string<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert the integer to a string and serialize it as a string
    serializer.serialize_str(&pubkey.to_string())
}

fn deserialize_pubkey_from_string<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSignature {
    pub details: String,
    pub nonce: String,
    pub signature: String,
    #[serde(
        serialize_with = "serialize_pubkey_as_string",
        deserialize_with = "deserialize_pubkey_from_string"
    )]
    pub pubkey: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointNodeToProviderNodeHeader {
    pub nonce: String,
    pub signature: String,
    #[serde(
        serialize_with = "serialize_pubkey_as_string",
        deserialize_with = "deserialize_pubkey_from_string"
    )]
    pub pubkey: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRouteHeader {
    #[serde(
        serialize_with = "serialize_pubkey_as_string",
        deserialize_with = "deserialize_pubkey_from_string"
    )]
    pub api_token: Pubkey,
    pub client_signature: NodeSignature,
    pub provider_node_signature: Option<NodeSignature>,
    pub endpoint_node_signature: Option<NodeSignature>,
}

impl FullRouteHeader {
    pub fn new(
        nonce: String,
        signed_message: String,
        pubkey: Pubkey,
        api_token: Pubkey,
        details: String,
    ) -> Self {
        let client_signature: NodeSignature = NodeSignature {
            details: details.clone(),
            nonce: nonce.clone(),
            signature: signed_message.clone(),
            pubkey,
        };
        Self {
            client_signature,
            api_token,
            provider_node_signature: None,
            endpoint_node_signature: None,
        }
    }

    pub fn prepare_for_memo(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        output.push(format!("api_token: {}", self.api_token.to_string()));
        output.push(format!(
            "client_signature: {}",
            serde_json::to_string(&self.client_signature).unwrap()
        ));
        if let Some(provider_node_signature) = &self.provider_node_signature {
            output.push(format!(
                "provider_node_signature: {}",
                serde_json::to_string(provider_node_signature).unwrap()
            ));
        }
        if let Some(endpoint_node_signature) = &self.endpoint_node_signature {
            output.push(format!(
                "endpoint_node_signature: {}",
                serde_json::to_string(endpoint_node_signature).unwrap()
            ));
        }
        output
    }

    pub fn add_provider_node_signature(
        &mut self,
        nonce: String,
        signed_message: String,
        pubkey: Pubkey,
        details: String,
    ) {
        if self.provider_node_signature.is_none() {
            self.provider_node_signature = Some(NodeSignature {
                details,
                nonce,
                signature: signed_message,
                pubkey,
            });
        }
    }

    pub fn add_endpoint_node_signature(
        &mut self,
        nonce: String,
        signed_message: String,
        pubkey: Pubkey,
        details: String,
    ) {
        if self.endpoint_node_signature.is_none() {
            self.endpoint_node_signature = Some(NodeSignature {
                details,
                nonce,
                signature: signed_message,
                pubkey,
            });
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
            endpoint_node: None,
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

    #[tracing::instrument(name = "create_endpoint_account_if_needed", skip(self), ret, err)]
    pub async fn create_endpoint_account_if_needed(&mut self) -> anyhow::Result<()> {
        let endpoint_address = get_endpoint_address(&self.program_id, &self.get_pubkey());
        self.endpoint_node = Some(endpoint_address.0);
        let account = get_account(&self.rpc_client, &endpoint_address.0)
            .await
            .map_err(|e| {
                anyhow!(
                    "create_endpoint_account_if_needed::Error getting client account: {}",
                    e.to_string()
                )
            })?;
        match account {
            Some(_) => {
                tracing::info!(
                    "create_endpoint_account_if_needed::EndpointNode account already exists: {:?}",
                    &endpoint_address.0.to_string()
                );
                Ok(())
            }
            None => {
                let instruction =
                    create_endpoint_node(self.program_id, self.get_pubkey(), endpoint_address.0);
                let signature = build_txn_and_send_and_confirm(
                    &self.rpc_client,
                    vec![instruction],
                    &self.get_pubkey(),
                    &self.get_keypair(),
                )
                .await?;
                tracing::info!(
                    "create_endpoint_account_if_needed::EndpointNode account created: {}",
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

    #[tracing::instrument(name = "send_memo", skip(self), ret, err)]
    pub async fn send_memos(&self, memos: Vec<String>) -> anyhow::Result<()> {
        let instructions = memos
            .iter()
            .map(|memo| build_memo(memo.as_bytes(), &[]))
            .collect();
        let signature = build_txn_and_send_and_confirm(
            &self.rpc_client,
            instructions,
            &self.get_pubkey(),
            &self.get_keypair(),
        )
        .await?;
        tracing::info!("send_memo::Transaction sent: {}", signature);
        Ok(())
    }
}
