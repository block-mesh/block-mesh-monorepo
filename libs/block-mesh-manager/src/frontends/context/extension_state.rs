use std::cell::RefCell;

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::str::FromStr;

use chrono::Utc;
use gloo_utils::format::JsValueSerdeExt;
use leptos::logging::log;
use leptos::*;
#[allow(unused_imports)]
use leptos_dom::tracing;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

use crate::frontends::utils::connectors::{
    ask_for_all_storage_values, onPostMessage, send_message_channel,
};
use block_mesh_common::chrome_storage::{AuthStatus, MessageKey, MessageType};

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct ExtensionContext {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub blockmesh_ws_url: RwSignal<String>,
    pub status: RwSignal<AuthStatus>,
    pub uptime: RwSignal<f64>,
    pub invite_code: RwSignal<String>,
    pub success: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
    pub download_speed: RwSignal<f64>,
    pub upload_speed: RwSignal<f64>,
    pub last_update: RwSignal<i64>,
    pub wallet_address: RwSignal<Option<String>>,
}

impl Default for ExtensionContext {
    fn default() -> Self {
        Self {
            email: RwSignal::new(String::default()),
            api_token: RwSignal::new(Uuid::default()),
            device_id: RwSignal::new(Uuid::default()),
            blockmesh_url: RwSignal::new("https://app.blockmesh.xyz".to_string()),
            blockmesh_ws_url: RwSignal::new("https://ws.blockmesh.xyz".to_string()),
            status: RwSignal::new(AuthStatus::LoggedOut),
            uptime: RwSignal::new(0.0),
            invite_code: RwSignal::new(String::default()),
            success: RwSignal::new(None),
            error: RwSignal::new(None),
            download_speed: RwSignal::new(0.0),
            upload_speed: Default::default(),
            last_update: RwSignal::new(0),
            wallet_address: RwSignal::new(None),
        }
    }
}

impl Debug for ExtensionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtensionState")
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
            .field("wallet_address", &self.wallet_address.get_untracked())
            .finish()
    }
}

impl ExtensionContext {
    pub fn init_resource(state: ExtensionContext) -> Resource<(), ExtensionContext> {
        create_local_resource(
            || (),
            move |_| async move {
                log!("ExtensionState => init_resource");
                state.init_with_storage().await;
                ask_for_all_storage_values().await;
                state
            },
        )
    }

    pub async fn init_with_storage(self) {
        let now = Utc::now().timestamp();
        let mut blockmesh_url = self.blockmesh_url.get_untracked();
        if blockmesh_url.is_empty() {
            blockmesh_url = "https://app.blockmesh.xyz".to_string();
            self.blockmesh_url.update(|v| *v = blockmesh_url.clone());
        }
        let email = self.email.get_untracked();
        let api_token = self.api_token.get_untracked();
        let mut device_id = self.device_id.get_untracked();
        if device_id.is_nil() || device_id == Uuid::default() {
            device_id = Uuid::new_v4();
            self.device_id.update(|v| *v = device_id);
        }

        let uptime = self.uptime.get_untracked();
        let invite_code = "".to_string();
        let download_speed = self.download_speed.get_untracked();
        let upload_speed = self.upload_speed.get_untracked();

        // Signals:
        self.invite_code.update(|v| *v = invite_code);
        self.email.update(|v| *v = email.clone());
        self.api_token.update(|v| *v = api_token);
        self.blockmesh_url.update(|v| *v = blockmesh_url.clone());
        self.status.update(|v| *v = AuthStatus::LoggedOut);
        self.device_id.update(|v| *v = device_id);
        self.uptime.update(|v| *v = uptime);
        self.success.update(|v| *v = None);
        self.error.update(|v| *v = None);
        self.download_speed.update(|v| *v = download_speed);
        self.upload_speed.update(|v| *v = upload_speed);
        self.last_update.update(|v| *v = now);
        self.wallet_address.update(|v| *v = None);
        let default_value: Value = Value::String("".to_string());

        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                if let Some(obj) = data.as_object() {
                    for key in obj.keys() {
                        if let Ok(storage_value) = MessageKey::try_from(key) {
                            if let Some(value) = obj.get(key) {
                                let value = if value.is_object() {
                                    value
                                        .as_object()
                                        .unwrap()
                                        .values()
                                        .next()
                                        .unwrap_or(&default_value)
                                        .to_string()
                                        .trim_end_matches('"')
                                        .trim_start_matches('"')
                                        .to_string()
                                } else if value.is_string() {
                                    value
                                        .to_string()
                                        .trim_end_matches('"')
                                        .trim_start_matches('"')
                                        .to_string()
                                } else {
                                    "".to_string()
                                };
                                match storage_value {
                                    MessageKey::BlockMeshUrl => {
                                        self.blockmesh_url.update(|v| *v = value);
                                    }
                                    MessageKey::BlockMeshWsUrl => {
                                        self.blockmesh_ws_url.update(|v| *v = value);
                                    }
                                    MessageKey::ApiToken => {
                                        self.api_token.update(|v| {
                                            *v = Uuid::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::Email => {
                                        self.email.update(|v| *v = value.to_ascii_lowercase());
                                    }
                                    MessageKey::DeviceId => {
                                        // setup_leptos_tracing(Option::from(device_id), DeviceType::Extension); // TODO
                                        self.device_id.update(|v| {
                                            *v = Uuid::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::Uptime => {
                                        self.uptime.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::InviteCode => {
                                        self.invite_code.update(|v| *v = value);
                                    }
                                    MessageKey::DownloadSpeed => {
                                        self.download_speed.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::UploadSpeed => {
                                        self.upload_speed.update(|v| {
                                            *v = f64::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::LastUpdate => self
                                        .last_update
                                        .update(|v| *v = i64::from_str(&value).unwrap_or_default()),
                                    MessageKey::WalletAddress => self.wallet_address.update(|v| {
                                        *v = (!value.is_empty()).then_some(value);
                                    }),
                                    MessageKey::All => {
                                        log!("GET_ALL");
                                    }
                                }

                                if !self.email.get_untracked().is_empty()
                                    && !self.api_token.get_untracked().is_nil()
                                    && self.api_token.get_untracked() != Uuid::default()
                                    && self.status.get_untracked() != AuthStatus::LoggedIn
                                {
                                    spawn_local(async move {
                                        self.status.update(|v| *v = AuthStatus::LoggedIn);
                                        // let credentials = CheckTokenRequest {
                                        //     api_token: self.api_token.get_untracked(),
                                        //     email: self.email.get_untracked(),
                                        // };
                                        // let result = check_token(
                                        //     &self.blockmesh_url.get_untracked(),
                                        //     &credentials,
                                        // )
                                        // .await;
                                        // if result.is_ok() {
                                        //     self.status.update(|v| *v = AuthStatus::LoggedIn);
                                        // };
                                    });
                                }
                            }
                        }
                    }
                }
            }
        });

        let closure_ref = Rc::new(RefCell::new(Some(callback)));
        let closure_clone = closure_ref.clone();
        onPostMessage(closure_clone.borrow().as_ref().unwrap());
        closure_ref.borrow_mut().take().unwrap().forget();
    }

    pub fn has_api_token(&self) -> bool {
        let api_token = self.api_token.get_untracked();
        !(api_token.is_nil() || api_token == Uuid::default())
    }

    pub async fn clear(&self) {
        send_message_channel(MessageType::DELETE, MessageKey::ApiToken, None).await;
        send_message_channel(MessageType::DELETE, MessageKey::Email, None).await;
        send_message_channel(MessageType::DELETE, MessageKey::BlockMeshUrl, None).await;
        self.blockmesh_url
            .update(|v| *v = "https://app.blockmesh.xyz".to_string());
        self.email.update(|v| *v = "".to_string());
        self.api_token.update(|v| *v = Uuid::default());
        self.status.update(|v| *v = AuthStatus::LoggedOut);
    }
}
