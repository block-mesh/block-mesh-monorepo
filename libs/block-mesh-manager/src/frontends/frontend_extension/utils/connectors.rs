use block_mesh_common::chrome_storage::{MessageKey, MessageType, MessageValue};
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
    export function storageOnChangeViaPostMessage(callback) {
        if (!window.message_channel_port) return;
        window.message_channel_port.addEventListener("message", (msg) => {
            const { data } = msg;
            callback(data);
        });
    }
"#)]
extern "C" {
    pub fn storageOnChangeViaPostMessage(callback: &Closure<dyn Fn(JsValue)>);
}

#[wasm_bindgen(inline_js = r#"
    export async function send_message(msg) {
        try {
            if (! window.message_channel_port ) {
                console.log("message_channel_port is missing");
                return;
            }
           window.message_channel_port.postMessage(msg);
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    pub async fn send_message(msg: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMessage {
    pub msg_type: MessageType,
    pub key: MessageKey,
    pub value: Option<MessageValue>,
}

pub async fn ask_for_all_storage_values() {
    let msg = PostMessage {
        msg_type: MessageType::GET_ALL,
        key: MessageKey::All,
        value: None,
    };
    if let Ok(js_args) = serde_wasm_bindgen::to_value(&msg) {
        send_message(js_args).await;
    }
}

pub async fn send_to_clipboard(link: &str) {
    let msg = PostMessage {
        msg_type: MessageType::COPY_TO_CLIPBOARD,
        key: MessageKey::InviteCode,
        value: Option::from(MessageValue::String(link.to_string())),
    };
    if let Ok(js_args) = serde_wasm_bindgen::to_value(&msg) {
        send_message(js_args).await;
    }
}

pub async fn send_message_channel(
    msg_type: MessageType,
    key: MessageKey,
    value: Option<MessageValue>,
) {
    let msg = PostMessage {
        msg_type,
        key,
        value,
    };
    if let Ok(js_args) = serde_wasm_bindgen::to_value(&msg) {
        send_message(js_args).await;
    }
}
