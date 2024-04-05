use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::{Parser, ValueEnum};

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
pub enum ResponseType {
    Json,
    Text,
}

impl FromStr for ResponseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ResponseType::Json),
            "text" => Ok(ResponseType::Text),
            _ => Err(format!("{} is not a valid response type", s)),
        }
    }
}

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

#[derive(Parser, Debug)]
pub struct ClientNodeCliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: Option<Pubkey>,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long, default_value = "https://ifconfig.me/all.json")]
    pub target: String,
    #[arg(long)]
    pub proxy_override: Option<String>,
    #[arg(value_enum, default_value_t = Mode::Cli)]
    pub mode: Mode,
}
