use crate::constants::BLOCK_MESH_PROGRAM_ID;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Main CLI arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Commands are mutually exclusive groups
#[derive(Subcommand, Debug)]
pub enum Commands {
    ClientNode(ClientNodeOptions),
    ProxyMaster(ProxyMasterNodeOptions),
    ProxyEndpoint(ProxyEndpointNodeOptions),
}

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
pub enum ClientNodeMode {
    Cli,
    Proxy,
}

impl FromStr for ClientNodeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(ClientNodeMode::Cli),
            "proxy" => Ok(ClientNodeMode::Proxy),
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
    #[arg(value_enum, default_value_t = ClientNodeMode::Cli)]
    /// Mode to run in (cli or proxy)
    pub mode: ClientNodeMode,
    #[arg(long, default_value = "8100")]
    /// Port to listen on, relevant for proxy mode only
    pub proxy_port: u16,
}

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
