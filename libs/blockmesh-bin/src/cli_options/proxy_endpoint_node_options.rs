use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Arguments for proxy-endpoint
#[derive(Parser, Debug)]
pub struct ProxyEndpointNodeOptions {
    /// Path to the keypair
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    /// Proxy-Master owner public key, by default will take first one found on-chain
    pub proxy_master_node_owner: Option<Pubkey>,
    /// BlockMesh Solana Program ID
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long)]
    /// Override the proxy-master URL, mostly for testing purposes
    pub proxy_override: Option<String>,
}
