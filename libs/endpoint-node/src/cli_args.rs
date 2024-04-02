use anchor_lang::prelude::Pubkey;
use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: Option<Pubkey>,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long)]
    pub proxy_override: Option<String>,
}
