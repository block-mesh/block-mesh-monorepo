use leptos::*;
use leptos_dom::tracing;
use leptos_router::use_navigate;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

use crate::pages::page::Page;
use crate::utils::auth::{check_token, get_latest_invite_code};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetLatestInviteCodeRequest};
use block_mesh_common::leptos_tracing::setup_leptos_tracing;
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
    pub invite_code: RwSignal<String>,
    pub success: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
    pub download_speed: RwSignal<f64>,
    pub upload_speed: RwSignal<f64>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("email", &self.email.get_untracked())
            .field("user_id", &self.device_id.get_untracked())
            .field("api_token", &"********")
            .field("blockmesh_url", &self.blockmesh_url.get_untracked())
            .field("uptime", &self.uptime.get_untracked())
            .field("status", &self.status.get_untracked())
            .field("invite_code", &self.invite_code.get_untracked())
            .field("success", &self.success.get_untracked())
            .field("error", &self.error.get_untracked())
            .field("download_speed", &self.download_speed.get_untracked())
            .field("upload_speed", &self.upload_speed.get_untracked())
            .finish()
    }
}

impl AppState {
    #[tracing::instrument(name = "AppState::new")]
    pub async fn new() -> Self {
        let mut blockmesh_url = Self::get_blockmesh_url().await;
        if blockmesh_url.is_empty() {
            blockmesh_url = "https://app.blockmesh.xyz".to_string();
            Self::store_blockmesh_url(blockmesh_url.clone()).await;
        }
        let email = Self::get_email().await;
        let api_token = Self::get_api_token().await;
        let api_token = uuid::Uuid::from_str(&api_token).unwrap_or_else(|_| Uuid::default());
        let mut device_id = Self::get_device_id().await;
        if device_id.is_empty() {
            device_id = Uuid::new_v4().to_string();
            Self::store_device_id(device_id.clone()).await;
        }
        let device_id = uuid::Uuid::from_str(&device_id).unwrap_or_else(|_| Uuid::new_v4());
        setup_leptos_tracing(Option::from(device_id), DeviceType::Extension);
        let uptime = Self::get_uptime().await;
        let mut invite_code = Self::get_invite_code().await;
        if !api_token.is_nil() && api_token != Uuid::default() {
            if let Ok(result) = get_latest_invite_code(
                &blockmesh_url,
                &GetLatestInviteCodeRequest {
                    email: email.clone(),
                    api_token,
                },
            )
            .await
            {
                invite_code = result.invite_code;
                Self::store_invite_code(invite_code.clone()).await;
            }
        }
        let download_speed = Self::get_download_speed().await;
        let upload_speed = Self::get_upload_speed().await;

        Self {
            invite_code: create_rw_signal(invite_code),
            email: create_rw_signal(email),
            api_token: create_rw_signal(api_token),
            blockmesh_url: create_rw_signal(blockmesh_url),
            status: create_rw_signal(AppStatus::LoggedOut),
            device_id: create_rw_signal(device_id),
            uptime: create_rw_signal(uptime),
            success: create_rw_signal(None),
            error: create_rw_signal(None),
            download_speed: create_rw_signal(download_speed),
            upload_speed: create_rw_signal(upload_speed),
        }
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

    pub fn has_api_token(&self) -> bool {
        let api_token = self.api_token.get_untracked();
        !(api_token.is_nil() || api_token == Uuid::default())
    }

    #[tracing::instrument(name = "AppState::init")]
    pub async fn init(context: AppState) -> AppStatus {
        let mut blockmesh_url = Self::get_blockmesh_url().await;
        if blockmesh_url.is_empty() {
            blockmesh_url = "https://app.blockmesh.xyz".to_string();
            Self::store_blockmesh_url(blockmesh_url.clone()).await;
        }
        let mut device_id = Self::get_device_id().await;
        if device_id.is_empty() {
            device_id = Uuid::new_v4().to_string();
            Self::store_device_id(device_id.clone()).await;
        }
        let device_id = uuid::Uuid::from_str(&device_id).unwrap_or_else(|_| Uuid::new_v4());
        setup_leptos_tracing(Option::from(device_id), DeviceType::Extension);
        let email = Self::get_email().await;
        let api_token = Self::get_api_token().await;
        let api_token = uuid::Uuid::from_str(&api_token).unwrap_or_else(|_| Uuid::default());
        let mut invite_code = Self::get_invite_code().await;
        if context.status.get_untracked() != AppStatus::LoggedOut && context.has_api_token() {
            if let Ok(result) = get_latest_invite_code(
                &blockmesh_url,
                &GetLatestInviteCodeRequest {
                    email: email.clone(),
                    api_token,
                },
            )
            .await
            {
                invite_code = result.invite_code;
                Self::store_invite_code(invite_code.clone()).await;
            }
        }
        let download_speed = Self::get_download_speed().await;
        let upload_speed = Self::get_upload_speed().await;

        context.upload_speed.update(|v| *v = upload_speed);
        context.download_speed.update(|v| *v = download_speed);
        context.invite_code.update(|v| *v = invite_code.clone());
        context.blockmesh_url.update(|v| *v = blockmesh_url.clone());
        context.email.update(|v| *v = email.clone());
        context.api_token.update(|v| *v = api_token);
        context.device_id.update(|v| *v = device_id);
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
    pub fn init_resource(context: AppState) {
        spawn_local(async move {
            let status = AppState::init(context).await;
            if status == AppStatus::LoggedIn {
                let navigate = use_navigate();
                navigate(Page::Home.path(), Default::default());
            }
        });
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

    pub async fn store_device_id(device_id: String) {
        set_storage_value(
            &StorageValues::DeviceId.to_string(),
            JsValue::from_str(&device_id),
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

    pub async fn store_invite_code(invite_code: String) {
        set_storage_value(
            &StorageValues::InviteCode.to_string(),
            JsValue::from_str(&invite_code),
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

    pub async fn store_download_speed(uptime: f64) {
        set_storage_value(
            &StorageValues::DownloadSpeed.to_string(),
            JsValue::from_f64(uptime),
        )
        .await;
    }

    pub async fn store_upload_speed(uptime: f64) {
        set_storage_value(
            &StorageValues::UploadSpeed.to_string(),
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

    pub async fn get_invite_code() -> String {
        get_storage_value(StorageValues::InviteCode.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }

    pub async fn get_uptime() -> f64 {
        let value = get_storage_value(StorageValues::Uptime.to_string().as_str()).await;
        let str = value.as_string().unwrap_or_default();
        f64::from_str(&str).unwrap_or_default()
    }

    pub async fn get_download_speed() -> f64 {
        let value = get_storage_value(StorageValues::DownloadSpeed.to_string().as_str()).await;
        let str = value.as_string().unwrap_or_default();
        f64::from_str(&str).unwrap_or_default()
    }

    pub async fn get_upload_speed() -> f64 {
        let value = get_storage_value(StorageValues::UploadSpeed.to_string().as_str()).await;
        let str = value.as_string().unwrap_or_default();
        f64::from_str(&str).unwrap_or_default()
    }
}
