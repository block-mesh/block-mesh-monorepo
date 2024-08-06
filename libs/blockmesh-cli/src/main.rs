use std::process::ExitCode;

use clap::Parser;

use block_mesh_common::cli::CliOpts;
use blockmesh_cli::run_cli;

mod helpers;

#[tokio::main]
pub async fn main() -> anyhow::Result<ExitCode> {
    let args = CliOpts::parse();
    run_cli(&args.email, &args.password).await?;
    Ok(ExitCode::SUCCESS)
}
