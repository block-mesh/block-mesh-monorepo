use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct ProviderNodeCliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long, default_value = BLOCK_MESH_PROGRAM_ID, value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
    #[arg(long, default_value = "3000")]
    pub port: u16,
    #[arg(long)]
    pub proxy_manager_address: String,
}
