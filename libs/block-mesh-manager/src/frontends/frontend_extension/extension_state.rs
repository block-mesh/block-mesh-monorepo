use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::str::FromStr;

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

use crate::frontends::frontend_extension::utils::auth::{check_token, get_latest_invite_code};
use crate::frontends::frontend_extension::utils::connectors::{
    ask_for_all_storage_values, send_message_channel, onPostMessage,
};
use block_mesh_common::chrome_storage::{AuthStatus, MessageKey, MessageType};
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetLatestInviteCodeRequest};

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
pub struct ExtensionState {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub status: RwSignal<AuthStatus>,
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
            .finish()
    }
}

impl ExtensionState {
    #[tracing::instrument(name = "init_resource")]
    pub fn init_resource(state: ExtensionState) -> Resource<(), ExtensionState> {
        create_local_resource(
            || (),
            move |_| async move {
                log!("init_resource");
                state.init_with_storage().await;
                ask_for_all_storage_values().await;
                state
            },
        )
    }

    #[tracing::instrument(name = "ExtensionState::init_with_storage")]
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

        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                // log!("data = {:#?}", data);
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
                                        .unwrap()
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
                                    MessageKey::ApiToken => {
                                        self.api_token.update(|v| {
                                            *v = Uuid::from_str(&value).unwrap_or_default()
                                        });
                                    }
                                    MessageKey::Email => {
                                        self.email.update(|v| *v = value);
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
                                        let credentials = CheckTokenRequest {
                                            api_token: self.api_token.get_untracked(),
                                            email: self.email.get_untracked(),
                                        };
                                        let result = check_token(
                                            &self.blockmesh_url.get_untracked(),
                                            &credentials,
                                        )
                                        .await;
                                        if result.is_ok() {
                                            self.status.update(|v| *v = AuthStatus::LoggedIn);
                                        };
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

    pub async fn update_invite_code(
        api_token: &Uuid,
        time_diff: i64,
        blockmesh_url: &str,
        email: &str,
    ) -> String {
        // let mut invite_code = Self::get_invite_code().await;
        let mut invite_code = "".to_string();
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
                // Self::store_invite_code(invite_code.clone()).await;
            }
        }
        invite_code
    }

    pub fn has_api_token(&self) -> bool {
        let api_token = self.api_token.get_untracked();
        !(api_token.is_nil() || api_token == Uuid::default())
    }

    #[tracing::instrument(name = "clear")]
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
