use anchor_lang::prelude::Pubkey;
use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long, default_value = "127.0.0.1")]
    pub ip: String,
    #[arg(long, default_value = "5000")]
    pub port: u16,
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: Pubkey,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long)]
    pub proxy_override: Option<String>,
}
