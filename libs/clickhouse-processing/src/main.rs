mod backup;

mod cli_opts;
mod export;
mod process;
mod s3_utils;
mod utils;

use crate::cli_opts::CliOpts;
use crate::cli_opts::CliOpts::{Export, Process};
use crate::export::export;
use crate::process::process;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliOpts::parse();
    println!("args = {:#?}", args);
    match args {
        Process(args) => process(args)
            .await
            .map_err(|e| eprintln!("process error {}", e))
            .unwrap(),
        Export(args) => export(args)
            .await
            .map_err(|e| eprintln!("export error {}", e))
            .unwrap(),
    }
    Ok(())
}
