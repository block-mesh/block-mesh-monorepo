use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct ProviderNodeCliArgs {
    #[arg(long)]
    pub keypair_path: String,
    #[arg(long)]
    pub provider_node_owner: String,
    #[arg(long, default_value = "CfaL9sdaEK49r4WLAtVh2vVgAZuv2eKbb6jSB5jDCMSF", value_parser = Pubkey::from_str)]
    pub program_id: Pubkey,
}
