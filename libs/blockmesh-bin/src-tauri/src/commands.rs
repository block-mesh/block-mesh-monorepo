use crate::system_tray::set_dock_visible;
use crate::tauri_state::{AppState, ChannelMessage};
use crate::tauri_storage::set_config_with_path;
use block_mesh_common::app_config::AppConfig;
use std::sync::Arc;
use tauri::{AppHandle, InvokeError, Manager, State};
use tokio::sync::Mutex;

#[tauri::command]
#[tracing::instrument(name = "get_app_config", skip(state), ret)]
pub async fn get_app_config(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, InvokeError> {
    let state = state.lock().await;
    let config = &state.config;
    serde_json::to_string(config).map_err(|e| InvokeError::from(e.to_string()))
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(name = "set_app_config", skip(state), ret)]
pub async fn set_app_config(
    state: State<'_, Arc<Mutex<AppState>>>,
    mut config: AppConfig,
) -> Result<(), InvokeError> {
    let mut state = state.lock().await;
    let path = state.config.config_path.clone();
    // config
    //     .validate_keypair()
    //     .await
    //     .map_err(|e| InvokeError::from(e.to_string()))?;
    config.config_path = path;
    state.config = config.clone();
    set_config_with_path(config)
        .await
        .ok_or(InvokeError::from("Error setting config"))?;
    state
        .tx
        .send(ChannelMessage::RestartTask)
        .map_err(|e| InvokeError::from(e.to_string()))?;
    Ok(())
}

#[tauri::command]
#[tracing::instrument(name = "open_main_window", skip(app_handle), ret, err)]
pub fn open_main_window(app_handle: &AppHandle) -> anyhow::Result<()> {
    set_dock_visible(true);
    if let Some(window) = app_handle.get_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let _window = tauri::WindowBuilder::new(
            app_handle,
            "main",
            tauri::WindowUrl::App("index.html".into()),
        )
        .visible(false)
        .build()?;
    }
    Ok(())
}
