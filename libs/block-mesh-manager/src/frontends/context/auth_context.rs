use crate::frontends::frontend_extension::utils::connectors::onPostMessage;
use block_mesh_common::chrome_storage::{
    AuthStatus, MessageKey, MessageType, MessageValue, PostMessage,
};
use gloo_utils::format::JsValueSerdeExt;
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
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

impl AuthContext {
    pub fn init_as_resource(state: AuthContext) -> Resource<(), AuthContext> {
        create_local_resource(
            || {},
            move |_| async move {
                state.init().await;
                state
            },
        )
    }
    #[tracing::instrument(name = "AuthContext::init")]
    pub async fn init(self) {
        let callback = Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
            if let Ok(data) = event.into_serde::<Value>() {
                if let Ok(msg) = PostMessage::try_from(data.clone()) {
                    match &msg.msg_type {
                        MessageType::SET => match msg.key {
                            MessageKey::Email => match msg.value {
                                Some(MessageValue::String(email)) => self.email.set(email.clone()),
                                _ => {}
                            },
                            MessageKey::ApiToken => match msg.value {
                                Some(MessageValue::UUID(uuid)) => self.api_token.set(uuid.clone()),
                                _ => {}
                            },
                            MessageKey::BlockMeshUrl => match msg.value {
                                Some(MessageValue::String(url)) => {
                                    self.blockmesh_url.set(url.clone())
                                }
                                _ => {}
                            },
                            MessageKey::DeviceId => match msg.value {
                                Some(MessageValue::UUID(device_id)) => {
                                    self.device_id.set(device_id.clone())
                                }
                                _ => {}
                            },
                            MessageKey::InviteCode => match msg.value {
                                Some(MessageValue::String(invite_code)) => {
                                    self.invite_code.set(invite_code.clone())
                                }
                                _ => {}
                            },
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
