use block_mesh_constants::BLOCK_MESH_PROGRAM_ID;
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

#[derive(Parser, Debug)]
pub struct ClientNodeCliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: Pubkey,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long, default_value = "https://api.ipify.org?format=json")]
    pub target: String,
    #[arg(value_enum, default_value_t = ResponseType::Json)]
    pub response_type: ResponseType,
}
