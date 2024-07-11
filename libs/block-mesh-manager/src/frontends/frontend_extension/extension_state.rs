use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;

use chrono::Utc;
use gloo_utils::format::JsValueSerdeExt;
use leptos::logging::log;
use leptos::*;
use leptos_dom::tracing;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetLatestInviteCodeRequest};
// use logger_leptos::leptos_tracing::setup_leptos_tracing; // TODO

use crate::frontends::frontend_extension::utils::auth::{check_token, get_latest_invite_code};
use crate::frontends::frontend_extension::utils::connectors::storageOnChangeViaPostMessage;
use block_mesh_common::chrome_storage::StorageValues;

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
pub struct ExtensionState {
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
    pub last_update: RwSignal<i64>,
}

impl Debug for ExtensionState {
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
            .field("last_update", &self.last_update.get_untracked())
            .finish()
    }
}

impl ExtensionState {
    #[tracing::instrument(name = "AppState::init_with_storage")]
    pub async fn init_with_storage(self) {
        let now = Utc::now().timestamp();
        let last_update = Self::get_last_update().await;
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
        // setup_leptos_tracing(Option::from(device_id), DeviceType::Extension);
        let uptime = Self::get_uptime().await;
        let invite_code =
            Self::update_invite_code(&api_token, now - last_update, &blockmesh_url, &email).await;
        let download_speed = Self::get_download_speed().await;
        let upload_speed = Self::get_upload_speed().await;

        Self::store_last_update(now).await;

        // Signals:
        self.invite_code.update(|v| *v = invite_code);
        self.email.update(|v| *v = email.clone());
        self.api_token.update(|v| *v = api_token);
        self.blockmesh_url.update(|v| *v = blockmesh_url.clone());
        self.status.update(|v| *v = AppStatus::LoggedOut);
        self.device_id.update(|v| *v = device_id);
        self.uptime.update(|v| *v = uptime);
        self.success.update(|v| *v = None);
        self.error.update(|v| *v = None);
        self.download_speed.update(|v| *v = download_speed);
        self.upload_speed.update(|v| *v = upload_speed);
        self.last_update.update(|v| *v = now);

        if !email.is_empty() && !api_token.is_nil() && api_token != Uuid::default() {
            let credentials = CheckTokenRequest { api_token, email };
            let result = check_token(&blockmesh_url, &credentials).await;
            if result.is_ok() {
                self.status.update(|v| *v = AppStatus::LoggedIn);
            };
        }

        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                log!("data {:?}", data);
                if let Some(obj) = data.as_object() {
                    log!("obj {:?}", data);
                    for key in obj.keys() {
                        log!("key {:?}", key);
                        if let Ok(storage_value) = StorageValues::try_from(key) {
                            log!("storage_value {:?}", storage_value);
                            if let Some(value) = obj.get(key) {
                                log!("in iframe {} => {}", storage_value, value);
                                let value = value.as_str().unwrap_or_default().to_string();
                                match storage_value {
                                    StorageValues::BlockMeshUrl => {
                                        self.blockmesh_url.update(|v| *v = value);
                                    }
                                    StorageValues::ApiToken => {
                                        self.api_token.update(|v| {
                                            *v = Uuid::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    StorageValues::Email => {
                                        self.email.update(|v| *v = value);
                                    }
                                    StorageValues::DeviceId => {
                                        self.device_id.update(|v| {
                                            *v = Uuid::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    StorageValues::Uptime => {
                                        self.uptime.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    StorageValues::InviteCode => {
                                        self.invite_code.update(|v| *v = value);
                                    }
                                    StorageValues::DownloadSpeed => {
                                        self.download_speed.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    StorageValues::UploadSpeed => {
                                        self.upload_speed.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    StorageValues::LastUpdate => self
                                        .last_update
                                        .update(|v| *v = i64::from_str(&value).unwrap_or_default()),
                                }
                            }
                        }
                    }
                }
            }
        });

        let closure_ref = Rc::new(RefCell::new(Some(callback)));
        let closure_clone = closure_ref.clone();
        storageOnChangeViaPostMessage(closure_clone.borrow().as_ref().unwrap());
        closure_ref.borrow_mut().take().unwrap().forget();
    }

    pub async fn update_invite_code(
        api_token: &Uuid,
        time_diff: i64,
        blockmesh_url: &str,
        email: &str,
    ) -> String {
        let mut invite_code = Self::get_invite_code().await;
        if !invite_code.is_empty() && time_diff < 600 {
            return invite_code;
        }
        if !api_token.is_nil() && *api_token != Uuid::default() {
            if let Ok(result) = get_latest_invite_code(
                blockmesh_url,
                &GetLatestInviteCodeRequest {
                    email: email.to_string(),
                    api_token: *api_token,
                },
            )
            .await
            {
                invite_code = result.invite_code;
                Self::store_invite_code(invite_code.clone()).await;
            }
        }
        invite_code
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

    #[tracing::instrument(name = "init_resource")]
    pub fn init_resource(state: ExtensionState) -> Resource<(), ExtensionState> {
        create_local_resource(
            || (),
            move |_| async move {
                state.init_with_storage().await;
                state
            },
        )
    }

    #[tracing::instrument(name = "clear")]
    pub async fn clear(&self) {
        self.blockmesh_url
            .update(|v| *v = "https://app.blockmesh.xyz".to_string());
        self.email.update(|v| *v = "".to_string());
        self.api_token.update(|v| *v = Uuid::default());
        self.status.update(|v| *v = AppStatus::LoggedOut);
        ExtensionState::store_api_token(Uuid::default()).await;
        ExtensionState::store_email("".to_string()).await;
        ExtensionState::store_blockmesh_url("https://app.blockmesh.xyz".to_string()).await;
    }

    pub async fn store_blockmesh_url(_blockmesh_url: String) {
        // set_storage_value(
        //     &StorageValues::BlockMeshUrl.to_string(),
        //     JsValue::from_str(&blockmesh_url),
        // )
        //     .await;
    }

    pub async fn store_device_id(_device_id: String) {
        // set_storage_value(
        //     &StorageValues::DeviceId.to_string(),
        //     JsValue::from_str(&device_id),
        // )
        //     .await;
    }

    pub async fn store_email(_email: String) {
        // set_storage_value(&StorageValues::Email.to_string(), JsValue::from_str(&email)).await;
    }

    pub async fn store_api_token(_api_token: Uuid) {
        // set_storage_value(
        //     &StorageValues::ApiToken.to_string(),
        //     JsValue::from_str(&api_token.to_string()),
        // )
        //     .await;
    }

    pub async fn store_invite_code(_invite_code: String) {
        // set_storage_value(
        //     &StorageValues::InviteCode.to_string(),
        //     JsValue::from_str(&invite_code),
        // )
        //     .await;
    }

    pub async fn store_uptime(_uptime: f64) {
        // set_storage_value(
        //     &StorageValues::Uptime.to_string(),
        //     JsValue::from_f64(uptime),
        // )
        //     .await;
    }

    pub async fn store_last_update(_last_update: i64) {
        // set_storage_value(
        //     &StorageValues::LastUpdate.to_string(),
        //     JsValue::from_f64(last_update as f64),
        // )
        //     .await;
    }

    pub async fn get_blockmesh_url() -> String {
        // get_storage_value(StorageValues::BlockMeshUrl.to_string().as_str())
        //     .await
        //     .as_string()
        //     .unwrap_or("https://app.blockmesh.xyz".to_string())
        "".to_string()
    }

    pub async fn get_email() -> String {
        // get_storage_value(StorageValues::Email.to_string().as_str())
        //     .await
        //     .as_string()
        //     .unwrap_or_default()
        "".to_string()
    }

    pub async fn get_last_update() -> i64 {
        // let value = get_storage_value(StorageValues::LastUpdate.to_string().as_str()).await;
        // let str = value.as_string().unwrap_or_default();
        // i64::from_str(&str).unwrap_or_default()
        0
    }

    pub async fn get_api_token() -> String {
        // get_storage_value(StorageValues::ApiToken.to_string().as_str())
        //     .await
        //     .as_string()
        //     .unwrap_or_default()
        "".to_string()
    }

    pub async fn get_device_id() -> String {
        // get_storage_value(StorageValues::DeviceId.to_string().as_str())
        //     .await
        //     .as_string()
        //     .unwrap_or_default()
        "".to_string()
    }

    pub async fn get_invite_code() -> String {
        // get_storage_value(StorageValues::InviteCode.to_string().as_str())
        //     .await
        //     .as_string()
        //     .unwrap_or_default()
        "".to_string()
    }

    pub async fn get_uptime() -> f64 {
        // let value = get_storage_value(StorageValues::Uptime.to_string().as_str()).await;
        // let str = value.as_string().unwrap_or_default();
        // f64::from_str(&str).unwrap_or_default()
        0.0
    }

    pub async fn get_download_speed() -> f64 {
        // let value = get_storage_value(StorageValues::DownloadSpeed.to_string().as_str()).await;
        // let str = value.as_string().unwrap_or_default();
        // f64::from_str(&str).unwrap_or_default()
        0.0
    }

    pub async fn get_upload_speed() -> f64 {
        // let value = get_storage_value(StorageValues::UploadSpeed.to_string().as_str()).await;
        // let str = value.as_string().unwrap_or_default();
        // f64::from_str(&str).unwrap_or_default()
        0.0
    }
}
