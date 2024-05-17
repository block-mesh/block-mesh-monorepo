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
    let state = app_state.clone();
    let task = tauri::async_runtime::spawn(async move {
        let app_state = state.clone();
        let config = app_state.lock().await.config.clone();
        let commands = Commands::from(config);
        drop(app_state);
        match &commands {
            Commands::ClientNode(client_node_options) => {
                client_node_main(client_node_options).await
            }
            Commands::ProxyMaster(proxy_master_node_options) => {
                proxy_master_main(proxy_master_node_options).await
            }
            Commands::ProxyEndpoint(proxy_endpoint_node_options) => {
                proxy_endpoint_main(proxy_endpoint_node_options).await
            }
        }
    });

    let app_state = app_state.clone();
    let _: tauri::async_runtime::JoinHandle<anyhow::Result<ExitCode>> =
        tauri::async_runtime::spawn(async move {
            let app_state = app_state.clone();
            while let Ok(_msg) = rx.recv().await {
                task.abort();
                start_task(app_state.clone(), rx.resubscribe());
            }
            Ok(ExitCode::SUCCESS)
        });
}
