#![allow(clippy::let_underscore_future)]
use crate::tauri_state::AppState;
use block_mesh_common::app_config::TaskStatus;
use block_mesh_common::cli::Commands;
use client_node::client_node_main;
use proxy_endpoint::proxy_endpoint_main;
use proxy_master::proxy_master_main;
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(dead_code)]
pub async fn shutdown_node(app_state: Arc<Mutex<AppState>>) {
    let mut state = app_state.lock().await;
    if let Some(handle) = &state.node_handle {
        handle.abort();
        state.node_handle = None;
        state.config.task_status = Option::from(TaskStatus::Off);
    }
}

#[allow(dead_code)]
pub async fn start_node(app_state: Arc<Mutex<AppState>>) {
    let app_state_c = app_state.clone();
    let mut state = app_state.lock().await;
    shutdown_node(app_state.clone()).await;
    let node_handle = tauri::async_runtime::spawn(async move {
        let mut state = app_state_c.lock().await;
        state.config.task_status = Option::from(TaskStatus::Running);
        let config = state.config.clone();
        let commands = Commands::from(config);
        drop(state);
        let future = match &commands {
            Commands::ClientNode(client_node_options) => {
                client_node_main(client_node_options).await
            }
            Commands::ProxyMaster(proxy_master_node_options) => {
                proxy_master_main(proxy_master_node_options).await
            }
            Commands::ProxyEndpoint(proxy_endpoint_node_options) => {
                proxy_endpoint_main(proxy_endpoint_node_options).await
            }
        };
        log::info!("Task finished with status: {:?}", future);
        let mut state = app_state_c.lock().await;
        state.config.task_status = Option::from(TaskStatus::Off);
        // future
    });
    state.node_handle = Some(node_handle);
}
