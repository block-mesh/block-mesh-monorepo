use std::fmt;
use std::fmt::{Debug, Formatter};

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

use crate::frontends::utils::connectors::{ask_for_all_storage_values, onPostMessage};
use block_mesh_common::chrome_storage::{
    AuthStatus, MessageKey, MessageType, MessageValue, PostMessage,
};
use block_mesh_common::interfaces::server_api::AuthStatusResponse;

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct AuthContext {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub invite_code: RwSignal<String>,
    pub status: RwSignal<AuthStatus>,
    pub wallet_address: RwSignal<Option<String>>,
}

impl Debug for AuthContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthContext")
            .field("email", &self.email.get_untracked())
            .field("device_id", &self.device_id.get_untracked())
            .field("api_token", &"********")
            .field("blockmesh_url", &self.blockmesh_url.get_untracked())
            .field("invite_code", &self.invite_code.get_untracked())
            .field("status", &self.status.get_untracked())
            .field("wallet_address", &self.wallet_address.get_untracked())
            .finish()
    }
}

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            email: RwSignal::new(String::default()),
            api_token: RwSignal::new(Uuid::default()),
            device_id: RwSignal::new(Uuid::default()),
            blockmesh_url: RwSignal::new("https://app.blockmesh.xyz".to_string()),
            status: RwSignal::new(AuthStatus::LoggedOut),
            invite_code: RwSignal::new(String::default()),
            wallet_address: RwSignal::new(Default::default()),
        }
    }
}

impl AuthContext {
    pub fn init_as_resource(state: AuthContext) -> Resource<(), AuthContext> {
        create_local_resource(
            || {},
            move |_| async move {
                log!("AuthContext => init_resource");
                state.init().await;
                ask_for_all_storage_values().await;
                state
            },
        )
    }
    pub async fn init(self) {
        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                if let Ok(msg) = PostMessage::try_from(data.clone()) {
                    match &msg.msg_type {
                        MessageType::SET => match (msg.key, msg.value) {
                            (MessageKey::Email, Some(MessageValue::String(email))) => {
                                self.email.set(email.clone().to_ascii_lowercase());
                            }

                            (MessageKey::ApiToken, Some(MessageValue::UUID(uuid))) => {
                                self.api_token.set(uuid);
                            }

                            (MessageKey::BlockMeshUrl, Some(MessageValue::String(url))) => {
                                self.blockmesh_url.set(url.clone())
                            }

                            (MessageKey::DeviceId, Some(MessageValue::UUID(device_id))) => {
                                self.device_id.set(device_id)
                            }

                            (MessageKey::InviteCode, Some(MessageValue::String(invite_code))) => {
                                self.invite_code.set(invite_code.clone())
                            }

                            (
                                MessageKey::WalletAddress,
                                Some(MessageValue::String(wallet_address)),
                            ) => self.wallet_address.set(Some(wallet_address.clone())),
                            _ => {}
                        },
                        _ => log!("msg no need? {:?}", msg),
                    }
                }

                if let Some(obj) = data.as_object() {
                    for key in obj.keys() {
                        println!("AuthContext::init => key = {}", key);
                    }
                }
            }
        });

        onPostMessage(&callback);
        callback.forget();
    }

    pub async fn load_account_data() -> Result<AuthStatusResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/auth_status", window().origin()))
            .send()
            .await?;

        response.json::<AuthStatusResponse>().await
    }
}
