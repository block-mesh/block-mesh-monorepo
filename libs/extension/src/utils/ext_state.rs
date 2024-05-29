use leptos::*;
use leptos_router::use_navigate;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::pages::page::Page;
use crate::utils::auth::check_token;
use block_mesh_common::interface::CheckTokenRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::JsValue;

use crate::utils::connectors::{get_storage_value, set_storage_value};
use crate::utils::storage::StorageValues;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum AppStatus {
    LoggedIn,
    LoggedOut,
    WaitingEmailVerification,
}

impl Display for AppStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppStatus::LoggedIn => write!(f, "LoggedIn"),
            AppStatus::LoggedOut => write!(f, "LoggedOut"),
            AppStatus::WaitingEmailVerification => write!(f, "WaitingEmailVerification"),
        }
    }
}

impl Default for AppStatus {
    fn default() -> Self {
        Self::LoggedOut
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
pub struct AppState {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub status: RwSignal<AppStatus>,
    pub uptime: RwSignal<f64>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("email", &self.email.get_untracked())
            .field("user_id", &self.device_id.get_untracked())
            .field("api_token", &self.api_token.get_untracked())
            .field("blockmesh_url", &self.blockmesh_url.get_untracked())
            .field("uptime", &self.uptime.get_untracked())
            .field("status", &self.status.get_untracked())
            .finish()
    }
}

impl AppState {
    #[tracing::instrument(name = "AppState::new")]
    pub async fn new() -> Self {
        let blockmesh_url = Self::get_blockmesh_url().await;
        let email = Self::get_email().await;
        let api_token = Self::get_api_token().await;
        let api_token = uuid::Uuid::from_str(&api_token).unwrap_or_else(|_| Uuid::default());
        let device_id = Self::get_device_id().await;
        let device_id = uuid::Uuid::from_str(&device_id).unwrap_or_else(|_| Uuid::new_v4());
        let uptime = Self::get_uptime().await;
        Self {
            email: create_rw_signal(email),
            api_token: create_rw_signal(api_token),
            blockmesh_url: create_rw_signal(blockmesh_url),
            status: create_rw_signal(AppStatus::LoggedOut),
            device_id: create_rw_signal(device_id),
            uptime: create_rw_signal(uptime),
        }
    }

    pub fn has_api_token(&self) -> bool {
        let api_token = self.api_token.get_untracked();
        !(api_token.is_nil() || api_token == Uuid::default())
    }

    #[tracing::instrument(name = "AppState::init")]
    pub async fn init(context: AppState) -> AppStatus {
        let blockmesh_url = Self::get_blockmesh_url().await;
        let email = Self::get_email().await;
        let api_token = Self::get_api_token().await;
        let api_token = uuid::Uuid::from_str(&api_token).unwrap_or_else(|_| Uuid::default());
        context.blockmesh_url.update(|v| *v = blockmesh_url.clone());
        context.email.update(|v| *v = email.clone());
        context.api_token.update(|v| *v = api_token);
        if email.is_empty() || api_token.is_nil() || api_token == Uuid::default() {
            return context.status.get_untracked();
        }
        let credentials = CheckTokenRequest { api_token, email };
        let result = check_token(&blockmesh_url, &credentials).await;
        if result.is_ok() {
            context.status.update(|v| *v = AppStatus::LoggedIn);
        };
        context.status.get_untracked()
    }
    #[tracing::instrument(name = "init_resource")]
    pub fn init_resource(context: AppState) -> Option<()> {
        let resource = create_resource(
            move || {
                (
                    context.blockmesh_url.get(),
                    context.email.get(),
                    context.api_token.get(),
                )
            },
            |_| async move {
                let state = use_context::<AppState>().unwrap();
                let status = AppState::init(state).await;
                if status == AppStatus::LoggedIn {
                    let navigate = use_navigate();
                    navigate(Page::Home.path(), Default::default());
                }
            },
        );
        resource.get()
    }

    #[tracing::instrument(name = "clear")]
    pub async fn clear(&self) {
        self.blockmesh_url
            .update(|v| *v = "https://app.blockmesh.xyz".to_string());
        self.email.update(|v| *v = "".to_string());
        self.api_token.update(|v| *v = Uuid::default());
        self.status.update(|v| *v = AppStatus::LoggedOut);
        AppState::store_api_token(Uuid::default()).await;
        AppState::store_email("".to_string()).await;
        AppState::store_blockmesh_url("https://app.blockmesh.xyz".to_string()).await;
    }

    pub async fn store_blockmesh_url(blockmesh_url: String) {
        set_storage_value(
            &StorageValues::BlockMeshUrl.to_string(),
            JsValue::from_str(&blockmesh_url),
        )
        .await;
    }

    pub async fn store_email(email: String) {
        set_storage_value(&StorageValues::Email.to_string(), JsValue::from_str(&email)).await;
    }

    pub async fn store_api_token(api_token: Uuid) {
        set_storage_value(
            &StorageValues::ApiToken.to_string(),
            JsValue::from_str(&api_token.to_string()),
        )
        .await;
    }

    pub async fn store_uptime(uptime: f64) {
        set_storage_value(
            &StorageValues::Uptime.to_string(),
            JsValue::from_f64(uptime),
        )
        .await;
    }

    pub async fn get_blockmesh_url() -> String {
        get_storage_value(StorageValues::BlockMeshUrl.to_string().as_str())
            .await
            .as_string()
            .unwrap_or("https://app.blockmesh.xyz".to_string())
    }

    pub async fn get_email() -> String {
        get_storage_value(StorageValues::Email.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }

    pub async fn get_api_token() -> String {
        get_storage_value(StorageValues::ApiToken.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }

    pub async fn get_device_id() -> String {
        get_storage_value(StorageValues::DeviceId.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }

    pub async fn get_uptime() -> f64 {
        get_storage_value(StorageValues::Uptime.to_string().as_str())
            .await
            .as_f64()
            .unwrap_or_default()
    }
}
