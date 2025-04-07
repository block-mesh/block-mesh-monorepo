use crate::cli::CliCommand;
use crate::rama_state::RamaState;
use crate::server::run;
use clap::Parser;

mod cli;
mod db;
pub mod error;
mod rama_state;
mod routes;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = CliCommand::parse();
    let state = RamaState::new().await?;
    run(cfg, state).await?;
    Ok(())
}
