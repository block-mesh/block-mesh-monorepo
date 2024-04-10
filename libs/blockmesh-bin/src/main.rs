use clap::{Parser, Subcommand};
mod cli_options;
use crate::cli_options::client_node_options::ClientNodeOptions;
use crate::cli_options::proxy_endpoint_node_options::ProxyEndpointNodeOptions;
use crate::cli_options::proxy_master_node_options::ProxyMasterNodeOptions;

/// Main CLI arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// Commands are mutually exclusive groups
#[derive(Subcommand, Debug)]
enum Commands {
    ClientNode(ClientNodeOptions),
    ProxyMaster(ProxyMasterNodeOptions),
    ProxyEndpoint(ProxyEndpointNodeOptions),
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::ClientNode(_a) => {}
        Commands::ProxyMaster(_b) => {}
        Commands::ProxyEndpoint(_c) => {}
    }
}
