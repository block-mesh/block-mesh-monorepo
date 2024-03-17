use anchor_lang::solana_program::message::Message;
use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::transaction::Transaction;
use std::str;
use std::str::FromStr;
use std::sync::Arc;

pub fn get_api_token_address(
    program_id: &Pubkey,
    client: &Pubkey,
    provider_node: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"API_TOKEN", &client.to_bytes(), &provider_node.to_bytes()],
        program_id,
    )
}

pub fn get_provider_node_address(program_id: &Pubkey, provider_node: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"PROVIDER_NODE", &provider_node.to_bytes()], program_id)
}

pub fn get_client_address(program_id: &Pubkey, client: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"CLIENT", &client.to_bytes()], program_id)
}

pub const PROGRAM_ID: &str = "k8hfCF1y8dP1hRXAHwmWke3bMYS5fMQ3kU4Tm8cdbGg";
const DEV_NET_HTTP: &str = "https://api.devnet.solana.com";

pub fn get_client() -> RpcClient {
    let url = DEV_NET_HTTP.to_string();
    RpcClient::new_with_commitment(url.clone(), CommitmentConfig::processed())
}

pub fn create_transaction(
    instructions: Vec<Instruction>,
    payer: &Pubkey,
    signer: &Keypair,
    latest_blockhash: Hash,
) -> anyhow::Result<Transaction> {
    let signing_keypairs = vec![&signer];
    let mut transaction = Transaction::new_unsigned(Message::new(&instructions, Some(payer)));
    transaction
        .try_sign(&signing_keypairs, latest_blockhash)
        .map_err(|err| anyhow!("error: failed to sign transaction: {}", err))?;
    Ok(transaction)
}

pub async fn get_account(
    client: &Arc<RpcClient>,
    pubkey: &Pubkey,
) -> anyhow::Result<Option<Account>> {
    let account = client
        .get_account_with_commitment(pubkey, CommitmentConfig::processed())
        .await
        .map_err(|e| anyhow!("error: Solana: get_account failed: {}", e))?;
    Ok(account.value)
}

pub async fn get_recent_blockhash(client: &Arc<RpcClient>) -> anyhow::Result<Hash> {
    client
        .get_latest_blockhash()
        .await
        .map_err(|e| anyhow!("error: Solana: get_recent_blockhash failed: {}", e))
}

pub async fn build_txn_and_send_and_confirm(
    client: &Arc<RpcClient>,
    instructions: Vec<Instruction>,
    payer: &Pubkey,
    signer: &Keypair,
) -> anyhow::Result<Signature> {
    let latest_blockhash = get_recent_blockhash(client).await?;
    let txn = create_transaction(instructions, payer, signer, latest_blockhash)?;
    let signature = send_and_confirm_transaction(client, &txn)
        .await
        .map_err(|e| {
            anyhow!(
                "build_txn_and_send_and_confirm::Error sending transaction: {}",
                e.to_string()
            )
        })?;
    Ok(signature)
}

pub async fn send_and_confirm_transaction(
    client: &Arc<RpcClient>,
    txn: &Transaction,
) -> anyhow::Result<Signature> {
    match client.send_and_confirm_transaction(txn).await {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig).await {
                if confirmed {
                    return Ok(sig);
                }
            }
        },
        Err(e) => Err(anyhow!("error: send_and_confirm_transaction failed: {}", e)),
    }
}

pub fn sign_message(message: &str, keypair: &Keypair) -> anyhow::Result<String> {
    let signature = keypair.try_sign_message(message.as_bytes())?;
    Ok(signature.to_string())
}

pub fn validate_signature(
    message: &str,
    signature: &str,
    public_key: &Pubkey,
) -> anyhow::Result<bool> {
    let signature = Signature::from_str(signature)?;
    Ok(signature.verify(&public_key.to_bytes(), message.as_bytes()))
}

pub struct CloneableKeypair(Keypair);

impl CloneableKeypair {
    pub fn new(keypair: Keypair) -> Self {
        Self(keypair)
    }

    pub fn pubkey(&self) -> Pubkey {
        self.0.pubkey()
    }

    pub fn keypair(&self) -> Keypair {
        let clone = self.clone();
        clone.0
    }
}

impl Clone for CloneableKeypair {
    fn clone(&self) -> Self {
        let bytes = self.0.to_bytes();
        let keypair = Keypair::from_bytes(&bytes).unwrap();
        Self(keypair)
    }
}
