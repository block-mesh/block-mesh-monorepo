use crate::system_tray::set_dock_visible;
use crate::tauri_state::{AppState, ChannelMessage};
use crate::tauri_storage::set_config_with_path;
use crate::{APP_ENVIRONMENT, CHANNEL_MSG_TX, DEV_ENV};
use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::constants::BLOCK_MESH_APP_SERVER;
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, GetTokenResponse, LoginForm, RegisterForm, RegisterResponse,
};
use std::str::FromStr;
use std::sync::Arc;
use tauri::ipc::InvokeError;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use uuid::Uuid;

#[tauri::command]
#[tracing::instrument(name = "get_app_config", skip(state), ret)]
pub async fn get_app_config(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, InvokeError> {
    let state = state.lock().await;
    let config = &state.config;
    serde_json::to_string(config).map_err(|e| InvokeError::from(e.to_string()))
}

#[tauri::command]
#[tracing::instrument(name = "get_task_status", level = "trace", skip(state), ret)]
pub async fn get_task_status(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, InvokeError> {
    let state = state.lock().await;
    let config = &state.config;
    match &config.task_status {
        None => Ok(TaskStatus::Off.to_string()),
        Some(task_status) => Ok(task_status.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(name = "set_app_config", skip(state), ret)]
pub async fn set_app_config(
    state: State<'_, Arc<Mutex<AppState>>>,
    mut config: AppConfig,
) -> Result<(), InvokeError> {
    let mut state = state.lock().await;
    let path = state.config.config_path.clone();
    config
        .validate_keypair()
        .await
        .map_err(|e| InvokeError::from(e.to_string()))?;
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
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let _window = tauri::WebviewWindowBuilder::new(
            app_handle,
            "main",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .visible(false)
        .build()?;
    }
    Ok(())
}

#[tauri::command]
#[tracing::instrument(name = "toggle_miner", ret)]
pub async fn toggle_miner(
    // state: State<'_, Arc<Mutex<AppState>>>,
    task_status: TaskStatus,
) -> Result<(), InvokeError> {
    let tx = CHANNEL_MSG_TX.get().unwrap();
    match task_status {
        TaskStatus::Running => {
            let _ = tx.send(ChannelMessage::StartOre);
        }
        TaskStatus::Off => {
            let _ = tx.send(ChannelMessage::ShutdownOre);
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(name = "get_ore_status", level = "trace", skip(state), ret)]
pub async fn get_ore_status(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, InvokeError> {
    let state = state.lock().await;
    let config = &state.config;
    match &config.ore_status {
        None => Ok(TaskStatus::Off.to_string()),
        Some(task_status) => Ok(task_status.to_string()),
    }
}

#[tauri::command]
#[tracing::instrument(name = "logout", skip(state), ret)]
pub async fn logout(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), InvokeError> {
    let mut state = state.lock().await;
    state.config.email = None;
    state.config.api_token = None;
    let config = state.config.clone();
    set_config_with_path(config)
        .await
        .ok_or(InvokeError::from("Error setting config"))?;
    Ok(())
}

#[tauri::command]
#[tracing::instrument(name = "login", skip(state, login_form), ret)]
pub async fn login(
    state: State<'_, Arc<Mutex<AppState>>>,
    login_form: LoginForm,
) -> Result<GetTokenResponse, InvokeError> {
    let url = format!("{}/api/get_token", BLOCK_MESH_APP_SERVER);
    let client = reqwest::Client::new();
    let response: GetTokenResponse = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&login_form)
        .send()
        .await
        .map_err(|e| InvokeError::from(e.to_string()))?
        .json()
        .await
        .map_err(|e| InvokeError::from(e.to_string()))?;
    let mut state = state.lock().await;
    if let Some(api_token) = response.api_token {
        state.config.email = Some(login_form.email.clone());
        state.config.api_token = Some(api_token.to_string());
        let config = state.config.clone();
        set_config_with_path(config)
            .await
            .ok_or(InvokeError::from("Error setting config"))?;
        let _ = CHANNEL_MSG_TX
            .get()
            .unwrap()
            .send(ChannelMessage::StartUptime);
        let _ = CHANNEL_MSG_TX
            .get()
            .unwrap()
            .send(ChannelMessage::StartTaskPull);
    }
    Ok(response)
}

#[tauri::command]
#[tracing::instrument(name = "check_token", skip(state), ret)]
pub async fn check_token(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<GetTokenResponse, InvokeError> {
    let state = state.lock().await;
    if state.config.email.is_none() {
        return Err(InvokeError::from("Cant find email"));
    }
    if state.config.api_token.is_none() {
        return Err(InvokeError::from("Cant find ApiToken"));
    }
    let email = state.config.email.clone().unwrap_or_default();
    let uuid = state.config.api_token.clone().unwrap_or_default();
    let api_token = Uuid::from_str(&uuid).unwrap_or_default();
    let credentials = CheckTokenRequest { email, api_token };
    let url = format!("{}/api/check_token", BLOCK_MESH_APP_SERVER);
    let client = reqwest::Client::new();
    let response: GetTokenResponse = client
        .post(&url)
        .json(&credentials)
        .send()
        .await
        .map_err(|e| InvokeError::from(format!("Error {}", e)))?
        .json()
        .await
        .map_err(|e| InvokeError::from(format!("Error {}", e)))?;
    let _ = CHANNEL_MSG_TX
        .get()
        .unwrap()
        .send(ChannelMessage::StartUptime);
    let _ = CHANNEL_MSG_TX
        .get()
        .unwrap()
        .send(ChannelMessage::StartTaskPull);
    Ok(response)
}

#[tauri::command]
#[tracing::instrument(name = "register", skip(state, register_form), ret)]
pub async fn register(
    state: State<'_, Arc<Mutex<AppState>>>,
    register_form: RegisterForm,
) -> Result<(), InvokeError> {
    let url = format!("{}/register_api", BLOCK_MESH_APP_SERVER);
    let client = reqwest::Client::new();
    tracing::info!("register_form = {:?}", register_form);
    let response = client
        .post(&url)
        .form(&register_form)
        .send()
        .await
        .map_err(|e| InvokeError::from(format!("Failed to register - {}", e)))?;
    let response: RegisterResponse = response
        .json()
        .await
        .map_err(|e| InvokeError::from(e.to_string()))?;
    let mut state = state.lock().await;
    if response.status_code == 200 {
        state.config.email = Some(register_form.email.clone());
        Ok(())
    } else {
        Err(InvokeError::from(
            response
                .error
                .unwrap_or_else(|| "Failed to register".to_string()),
        ))
    }
}

#[tauri::command]
#[tracing::instrument(name = "get_home_url", ret)]
pub async fn get_home_url() -> String {
    return if DEV_ENV.contains(&APP_ENVIRONMENT.get().unwrap().as_str()) {
        "http://localhost:8000/tauri".to_string()
    } else {
        "http://app.blockmesh.xyz/tauri".to_string()
    };
}
