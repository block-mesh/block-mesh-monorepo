use crate::state::AppState;
use anyhow::anyhow;
use block_mesh_common::app_config::AppConfig;
use block_mesh_common::constants::CONFIG_FILENAME;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

#[tracing::instrument(name = "storage_path", skip(app_handle), ret)]
pub fn storage_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let path = app_handle
        .path_resolver()
        .app_config_dir()?
        .join(CONFIG_FILENAME);
    Some(path)
}

#[tracing::instrument(name = "get_config", skip(app_handle), ret)]
pub async fn get_config(app_handle: &AppHandle) -> Option<AppConfig> {
    let path = storage_path(app_handle)?;
    create_dir_all(path.parent()?).await.ok()?;
    if tokio::fs::metadata(&path).await.is_err() {
        let contents = serde_json::to_string(&AppConfig::default()).ok()?;
        tokio::fs::write(&path, contents).await.ok()?;
    }
    let contents = tokio::fs::read_to_string(path).await.ok()?;
    serde_json::from_str(&contents).ok()
}

#[tracing::instrument(name = "set_config_with_handle", skip(app_handle), ret)]
pub async fn set_config_with_handle(app_handle: &AppHandle, config: AppConfig) -> Option<()> {
    let path = storage_path(app_handle)?;
    create_dir_all(path.parent()?).await.ok()?;
    let contents = serde_json::to_string_pretty(&config).ok()?;
    tokio::fs::write(path, contents).await.ok()?;
    Some(())
}

#[tracing::instrument(name = "set_config_with_path", ret)]
pub async fn set_config_with_path(config: AppConfig) -> Option<()> {
    let path = config.config_path.clone()?;
    let path = PathBuf::from(path);
    create_dir_all(path.parent()?).await.ok()?;
    let contents = serde_json::to_string_pretty(&config).ok()?;
    tokio::fs::write(path, contents).await.ok()?;
    Some(())
}

#[tracing::instrument(name = "setup_storage", skip(app_handle), ret, err)]
pub async fn setup_storage(app_handle: AppHandle) -> anyhow::Result<()> {
    let path = storage_path(&app_handle).ok_or_else(|| anyhow!("Error getting storage path"))?;
    let mut storage_config = get_config(&app_handle)
        .await
        .ok_or_else(|| anyhow!("Error getting config"))?;
    storage_config.config_path = Some(
        path.to_str()
            .ok_or(anyhow!("Failed to get buf string"))?
            .to_string(),
    );
    let state = app_handle.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().await;
    app_state.config.merge(storage_config);
    Ok(())
}
