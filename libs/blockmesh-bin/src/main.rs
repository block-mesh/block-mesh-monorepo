use clap::Parser;
use std::process::ExitCode;

use block_mesh_common::cli::{CliArgs, Commands};
use client_node::client_node_main;
use proxy_master::proxy_master_main;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let args = CliArgs::parse();

    match args.command {
        Commands::ClientNode(client_node_options) => client_node_main(client_node_options).await,
        Commands::ProxyMaster(proxy_master_node_options) => {
            proxy_master_main(proxy_master_node_options).await
        }
        Commands::ProxyEndpoint(_c) => Ok(ExitCode::SUCCESS),
    }
}
