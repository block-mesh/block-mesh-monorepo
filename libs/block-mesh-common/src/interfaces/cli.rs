use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone, PartialEq, Default)]
#[command(author = "BlockMesh Network", version, about)]
pub struct CliOpts {
    /// Email
    #[arg(long)]
    pub email: String,
    /// Password
    #[arg(long)]
    pub password: String,
    #[arg(value_enum, default_value_t = CliOptMod::Login)]
    /// Mode
    pub mode: CliOptMod,
    /// Server URL
    #[arg(long, default_value = "https://app.blockmesh.xyz")]
    pub url: String,
    #[arg(long, default_value = "blockmesh-cli")]
    pub invite_code: String,
    /// DePIN aggregator name
    #[arg(long)]
    pub depin_aggregator: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum, PartialEq, Default)]
pub enum CliOptMod {
    #[default]
    Login,
    Register,
    Dashboard,
}
