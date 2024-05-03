#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod gui;

use clap::Parser;
use std::process::ExitCode;

use crate::gui::run_gui;
use block_mesh_common::cli::{CliArgs, Commands};
use client_node::client_node_main;
use proxy_endpoint::proxy_endpoint_main;
use proxy_master::proxy_master_main;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let args = CliArgs::parse();
    match &args.command {
        Commands::ClientNode(client_node_options) => {
            if client_node_options.gui {
                run_gui(&args.command);
            }
            client_node_main(client_node_options).await
        }
        Commands::ProxyMaster(proxy_master_node_options) => {
            if proxy_master_node_options.gui {
                run_gui(&args.command);
            }
            proxy_master_main(proxy_master_node_options).await
        }
        Commands::ProxyEndpoint(proxy_endpoint_node_options) => {
            if proxy_endpoint_node_options.gui {
                run_gui(&args.command);
            }
            proxy_endpoint_main(proxy_endpoint_node_options).await
        }
        Commands::Nothing => {
            println!("No command provided");
            Ok(ExitCode::SUCCESS)
        }
    }
}
