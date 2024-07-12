use block_mesh_common::chrome_storage::{StorageMessageType, StorageValue, StorageValues};
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
    export function storageOnChangeViaPostMessage(callback) {
        console.log("msg in callback X");
        if (!window.message_channel_port) return;
        window.message_channel_port.addEventListener("message", (msg) => {
            const { data } = msg;
            console.log("msg", window.location.href, "in callback", data);
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
pub struct StorageMessage {
    pub msg_type: StorageMessageType,
    pub key: StorageValues,
    pub value: Option<StorageValue>,
}

pub async fn send_message_channel(
    msg_type: StorageMessageType,
    key: StorageValues,
    value: Option<StorageValue>,
) {
    let msg = StorageMessage {
        msg_type,
        key,
        value,
    };
    if let Ok(js_args) = serde_wasm_bindgen::to_value(&msg) {
        send_message(js_args).await;
    }
}
