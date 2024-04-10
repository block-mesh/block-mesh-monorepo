use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
pub enum Mode {
    Cli,
    Proxy,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(Mode::Cli),
            "proxy" => Ok(Mode::Proxy),
            _ => Err(format!("{} is not a valid mode", s)),
        }
    }
}

/// Arguments for client-node
#[derive(Parser, Debug)]
pub struct ClientNodeOptions {
    /// Path to the keypair
    #[arg(long)]
    pub keypair_path: String,
    /// Proxy-Master owner public key, by default will take first one found on-chain
    #[arg(long)]
    pub proxy_master_node_owner: Option<Pubkey>,
    /// BlockMesh Solana Program ID
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    /// Target URL to fetch in CLI mode
    #[arg(long, default_value = "https://ifconfig.me/all.json")]
    pub target: String,
    #[arg(long)]
    /// Override the proxy-master URL, mostly for testing purposes
    pub proxy_override: Option<String>,
    #[arg(value_enum, default_value_t = Mode::Cli)]
    /// Mode to run in (cli or proxy)
    pub mode: Mode,
    #[arg(long, default_value = "8100")]
    /// Port to listen on, relevant for proxy mode only
    pub proxy_port: u16,
}
