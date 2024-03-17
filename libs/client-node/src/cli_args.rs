use block_mesh_constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct ClientNodeCliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: String,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
}
