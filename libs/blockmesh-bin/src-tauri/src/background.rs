#![allow(clippy::let_underscore_future)]
use crate::state::{AppState, ChannelMessage};
use block_mesh_common::cli::Commands;
use client_node::client_node_main;
use proxy_endpoint::proxy_endpoint_main;
use proxy_master::proxy_master_main;
use std::process::ExitCode;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;

pub fn start_task(app_state: Arc<Mutex<AppState>>, mut rx: Receiver<ChannelMessage>) {
    println!("Starting task");
    let state = app_state.clone();
    let task = tauri::async_runtime::spawn(async move {
        let app_state = state.clone();
        let args = app_state.lock().await.cli_args.clone();
        drop(app_state);
        match &args.command {
            Some(commands) => match &commands {
                Commands::ClientNode(client_node_options) => {
                    client_node_main(client_node_options).await
                }
                Commands::ProxyMaster(proxy_master_node_options) => {
                    proxy_master_main(proxy_master_node_options).await
                }
                Commands::ProxyEndpoint(proxy_endpoint_node_options) => {
                    proxy_endpoint_main(proxy_endpoint_node_options).await
                }
            },
            None => {
                println!("No command provided");
                Ok(ExitCode::SUCCESS)
            }
        }
    });

    let app_state = app_state.clone();
    let _: tauri::async_runtime::JoinHandle<anyhow::Result<ExitCode>> =
        tauri::async_runtime::spawn(async move {
            let app_state = app_state.clone();
            println!("Waiting for message 2");
            while let Ok(msg) = rx.recv().await {
                println!("Received message 2: {:?}", msg);
                task.abort();
                start_task(app_state.clone(), rx.resubscribe());
            }
            Ok(ExitCode::SUCCESS)
        });
    println!("Exiting method");
}
