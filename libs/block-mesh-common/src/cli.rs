use crate::constants::BLOCK_MESH_PROGRAM_ID;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use std::str::FromStr;

/// Main CLI arguments
#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long)]
    pub minimized: bool,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// Commands are mutually exclusive groups
#[derive(Subcommand, Debug, Clone, PartialEq)]
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

impl From<Commands> for CommandsEnum {
    fn from(command: Commands) -> Self {
        match command {
            Commands::ClientNode(_) => CommandsEnum::ClientNode,
            Commands::ProxyMaster(_) => CommandsEnum::ProxyMaster,
            Commands::ProxyEndpoint(_) => CommandsEnum::ProxyEndpoint,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CommandsEnum {
    ClientNode,
    ProxyMaster,
    ProxyEndpoint,
}

impl Display for CommandsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandsEnum::ClientNode => write!(f, "Client Node"),
            CommandsEnum::ProxyMaster => write!(f, "Proxy Master"),
            CommandsEnum::ProxyEndpoint => write!(f, "Proxy Endpoint"),
        }
    }
}

impl Commands {
    pub fn convert(&self, target: &CommandsEnum) -> Option<Self> {
        match (&self, target) {
            (Commands::ClientNode(options), CommandsEnum::ProxyEndpoint) => {
                Some(Commands::ProxyEndpoint(ProxyEndpointNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    proxy_master_node_owner: None,
                    program_id: options.program_id,
                    proxy_override: None,
                    gui: options.gui,
                }))
            }
            (Commands::ClientNode(options), CommandsEnum::ProxyMaster) => {
                Some(Commands::ProxyMaster(ProxyMasterNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    program_id: options.program_id,
                    proxy_port: 5000,
                    client_port: 4000,
                    gui: options.gui,
                }))
            }
            (Commands::ProxyMaster(options), CommandsEnum::ClientNode) => {
                Some(Commands::ClientNode(ClientNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    program_id: options.program_id,
                    proxy_master_node_owner: None,
                    target: "".to_string(),
                    proxy_override: None,
                    mode: ClientNodeMode::Cli,
                    proxy_port: 8100,
                    gui: options.gui,
                }))
            }
            (Commands::ProxyMaster(options), CommandsEnum::ProxyEndpoint) => {
                Some(Commands::ProxyEndpoint(ProxyEndpointNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    proxy_master_node_owner: None,
                    program_id: options.program_id,
                    proxy_override: None,
                    gui: options.gui,
                }))
            }
            (Commands::ProxyEndpoint(options), CommandsEnum::ClientNode) => {
                Some(Commands::ClientNode(ClientNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    program_id: options.program_id,
                    proxy_master_node_owner: None,
                    target: "".to_string(),
                    proxy_override: None,
                    mode: ClientNodeMode::Cli,
                    proxy_port: 8100,
                    gui: options.gui,
                }))
            }
            (Commands::ProxyEndpoint(options), CommandsEnum::ProxyMaster) => {
                Some(Commands::ProxyMaster(ProxyMasterNodeOptions {
                    keypair_path: options.keypair_path.clone(),
                    program_id: options.program_id,
                    proxy_port: 5000,
                    client_port: 4000,
                    gui: options.gui,
                }))
            }
            _ => None,
        }
    }
}

/// Arguments for proxy-endpoint
#[derive(Parser, Debug, Clone, PartialEq, Default)]
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum, PartialEq, Default)]
pub enum ClientNodeMode {
    #[default]
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
#[derive(Parser, Debug, Clone, PartialEq, Default)]
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
#[derive(Parser, Debug, Clone, PartialEq, Default)]
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
