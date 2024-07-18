use crate::tauri_connector::connector::{invoke_tauri, send_message_channel};
use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::chrome_storage::{MessageKey, MessageType, MessageValue};
use block_mesh_common::cli::CommandsEnum;
use block_mesh_common::interfaces::server_api::GetTokenResponse;
use leptos::*;
use std::fmt::{Debug, Display};
use std::time::Duration;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct LeptosTauriAppState {
    pub app_config: RwSignal<AppConfig>,
    pub success: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
    pub logged_in: RwSignal<bool>,
}

impl Default for LeptosTauriAppState {
    fn default() -> Self {
        let app_config = create_rw_signal(AppConfig::default());
        app_config.update(|c| c.mode = Some(CommandsEnum::ClientNode));
        let logged_in = create_rw_signal(false);
        let success = create_rw_signal(None);
        let error = create_rw_signal(None);
        Self {
            app_config,
            logged_in,
            success,
            error,
        }
    }
}

impl LeptosTauriAppState {
    #[tracing::instrument(name = "init_app_config", skip(state))]
    pub async fn init_app_config(state: &LeptosTauriAppState) {
        let app_config_json = invoke_tauri("get_app_config", JsValue::NULL).await;
        let app_config_json = match app_config_json {
            Ok(app_config_json) => {
                if app_config_json.is_null() {
                    return;
                }
                app_config_json
            }
            Err(e) => {
                tracing::error!("error: {}", e);
                return;
            }
        };
        let app_config_json = app_config_json.as_string().unwrap();
        let mut app_config: AppConfig = serde_json::from_str(&app_config_json).unwrap();
        if app_config.mode.is_none() {
            app_config.mode = Some(CommandsEnum::ClientNode);
        }
        tracing::info!("Loaded app_config: {:?}", app_config);
        if app_config.email.is_some() {
            send_message_channel(
                MessageType::SET,
                MessageKey::Email,
                Option::from(MessageValue::String(app_config.email.clone().unwrap())),
            )
            .await;
        }

        if app_config.api_token.is_some() {
            send_message_channel(
                MessageType::SET,
                MessageKey::ApiToken,
                Option::from(MessageValue::String(app_config.api_token.clone().unwrap())),
            )
            .await;
        }
        state.app_config.set(app_config);
    }

    #[tracing::instrument(name = "check_token", skip(state))]
    pub async fn check_token(state: &LeptosTauriAppState) {
        match invoke_tauri("check_token", JsValue::NULL).await {
            Ok(result) => {
                if serde_wasm_bindgen::from_value::<GetTokenResponse>(result).is_ok() {
                    state.logged_in.update(|v| *v = true);
                }
            }
            Err(e) => {
                tracing::error!("check_token error {}", e);
                state.logged_in.update(|v| *v = false);
            }
        }
        tracing::info!("Login status {}", state.logged_in.get_untracked());
    }

    #[tracing::instrument(name = "get_task_status", skip(state))]
    pub async fn get_task_status(state: &LeptosTauriAppState) {
        let result = invoke_tauri("get_task_status", JsValue::NULL).await;
        if let Ok(result) = result {
            let result = result.as_string().unwrap();
            let task = TaskStatus::from(result);
            // set_task_status.set(task.to_string());
            state.app_config.update(|config| {
                config.task_status = Some(task);
            });
        }
        tracing::info!(
            "Task status {:?}",
            state.app_config.get_untracked().task_status
        );
    }

    #[tracing::instrument(name = "get_ore_status", skip(state))]
    pub async fn get_ore_status(state: &LeptosTauriAppState) {
        let result = invoke_tauri("get_ore_status", JsValue::NULL).await;
        if let Ok(result) = result {
            let result = result.as_string().unwrap();
            let ore_status = TaskStatus::from(result);
            state.app_config.update(|config| {
                config.ore_status = Some(ore_status);
            });
        }
        tracing::info!(
            "Ore status {:?}",
            state.app_config.get_untracked().ore_status
        );
    }

    #[tracing::instrument(name = "AppState::set_success")]
    pub fn set_success<T>(success: T, signal: RwSignal<Option<String>>)
    where
        T: Display + Clone + Into<String> + Debug,
    {
        let success = Option::from(success.clone().to_string());
        signal.update(|v| *v = success);
        set_timeout(
            move || {
                signal.update(|v| *v = None);
            },
            Duration::from_millis(3500),
        );
    }

    #[tracing::instrument(name = "AppState::set_error")]
    pub fn set_error<T>(error: T, signal: RwSignal<Option<String>>)
    where
        T: Display + Clone + Into<String> + Debug,
    {
        let error = Option::from(error.clone().to_string());
        signal.update(|v| *v = error);
        set_timeout(
            move || {
                signal.update(|v| *v = None);
            },
            Duration::from_millis(3500),
        );
    }
}
