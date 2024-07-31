use crate::frontends::frontend_extension::utils::connectors::{
    ask_for_all_storage_values, onPostMessage, send_message_channel,
};
use block_mesh_common::chrome_storage::{
    AuthStatus, MessageKey, MessageType, MessageValue, PostMessage,
};
use gloo_utils::format::JsValueSerdeExt;
use leptos::logging::log;
use leptos::*;
use leptos_dom::tracing;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct AuthContext {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub invite_code: RwSignal<String>,
    pub status: RwSignal<AuthStatus>,
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
            .finish()
    }
}

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            email: create_rw_signal(String::default()),
            api_token: create_rw_signal(Uuid::default()),
            device_id: create_rw_signal(Uuid::default()),
            blockmesh_url: create_rw_signal("https://app.blockmesh.xyz".to_string()),
            status: create_rw_signal(AuthStatus::LoggedOut),
            invite_code: create_rw_signal(String::default()),
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
    #[tracing::instrument(name = "AuthContext::init")]
    pub async fn init(self) {
        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                log!("DATA = {:#?}", data);
                if let Ok(msg) = PostMessage::try_from(data.clone()) {
                    match &msg.msg_type {
                        MessageType::SET => match msg.key {
                            MessageKey::Email => {
                                if let Some(MessageValue::String(email)) = msg.value {
                                    self.email.set(email.clone());
                                }
                            }
                            MessageKey::ApiToken => {
                                if let Some(MessageValue::UUID(uuid)) = msg.value {
                                    self.api_token.set(uuid);
                                }
                            }
                            MessageKey::BlockMeshUrl => {
                                if let Some(MessageValue::String(url)) = msg.value {
                                    log!("Setting URL {}", url);
                                    self.blockmesh_url.set(url.clone())
                                }
                            }
                            MessageKey::DeviceId => {
                                if let Some(MessageValue::UUID(device_id)) = msg.value {
                                    self.device_id.set(device_id)
                                }
                            }
                            MessageKey::InviteCode => {
                                if let Some(MessageValue::String(invite_code)) = msg.value {
                                    self.invite_code.set(invite_code.clone())
                                }
                            }
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
        let closure_ref = Rc::new(RefCell::new(Some(callback)));
        let closure_clone = closure_ref.clone();
        onPostMessage(closure_clone.borrow().as_ref().unwrap());
        closure_ref.borrow_mut().take().unwrap().forget();
    }
}
