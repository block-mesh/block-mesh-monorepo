use crate::constants::BLOCK_MESH_PROGRAM_ID;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use std::str::FromStr;

/// Main CLI arguments
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub minimized: bool,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// Commands are mutually exclusive groups
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    ClientNode(ClientNodeOptions),
    ProxyMaster(ProxyMasterNodeOptions),
    ProxyEndpoint(ProxyEndpointNodeOptions),
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::ClientNode(_) => write!(f, "ClientNode"),
            Commands::ProxyMaster(_) => write!(f, "ProxyMaster"),
            Commands::ProxyEndpoint(_) => write!(f, "ProxyEndpoint"),
        }
    }
}

/// Arguments for proxy-endpoint
#[derive(Parser, Debug, Clone)]
pub struct ProxyEndpointNodeOptions {
    /// Path to the keypair
    #[arg(long, default_value = "proxy-endpoint-keypair.json")]
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
    #[clap(long, short)]
    pub gui: bool,
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
#[derive(Parser, Debug, Clone)]
pub struct ClientNodeOptions {
    /// Path to the keypair
    #[arg(long, default_value = "client-keypair.json")]
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
    #[clap(long, short)]
    pub gui: bool,
}

/// Arguments for proxy-master
#[derive(Parser, Debug, Clone)]
pub struct ProxyMasterNodeOptions {
    /// Path to the keypair
    #[arg(long, default_value = "proxy-master-keypair.json")]
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
    #[clap(long, short)]
    pub gui: bool,
}
