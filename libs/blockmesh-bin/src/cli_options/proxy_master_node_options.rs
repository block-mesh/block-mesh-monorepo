use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Arguments for proxy-master
#[derive(Parser, Debug)]
pub struct ProxyMasterNodeOptions {
    /// Path to the keypair
    #[arg(long)]
    pub keypair_path: String,
    /// BlockMesh Solana Program ID
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    /// Port to listen for incoming proxy-endpoints
    #[arg(long, default_value = "5000")]
    pub proxy_port: u16,
    /// Port to listen for incoming clients
    #[arg(long, default_value = "4000")]
    pub client_port: u16,
}
