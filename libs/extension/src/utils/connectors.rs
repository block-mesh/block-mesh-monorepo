use leptos::spawn_local;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use block_mesh_common::chrome_storage::{MessageKey, MessageValue};

/// This is a proxy for report_progress() in progress.js
/// to send messages to other js scripts.
#[wasm_bindgen(inline_js = r#"
    export function report_progress(msg) {
        function onSuccess(message) {
            console.log(`report_progress::onSuccess: ${JSON.stringify(message)}`);
        }
        function onError(error) {
            console.log(`report_progress::onError: ${error}`);
        }
        try {
            chrome.runtime.sendMessage(msg).then(onSuccess, onError)
        } catch (e) {
            console.log('report_progress', { e })
        }
    }"#)]
extern "C" {
    pub fn report_progress(msg: &str);
}

#[wasm_bindgen(inline_js = r#"
    export async function send_to_iframe(key, value) {
        try {
            if (!window.message_channel_port) {
                console.log("message channel port missing");
                return;
            }
            window.message_channel_port.postMessage({[key]: value});
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    // need to rewrite with this: https://github.com/Pauan/tab-organizer/blob/rust/web-extension/src/storage.rs
    pub async fn send_to_iframe(key: &str, value: JsValue) -> JsValue;
}

pub fn send_storage_value_to_iframe(key: MessageKey, value: MessageValue) {
    if let Ok(js_args) = serde_wasm_bindgen::to_value(&value) {
        spawn_local(async move {
            send_to_iframe(&key.to_string(), js_args).await;
        })
    }
}

#[wasm_bindgen(inline_js = r#"
    export async function remove_storage_value(key) {
        try {
            await chrome.storage.sync.remove(key);
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    // need to rewrite with this: https://github.com/Pauan/tab-organizer/blob/rust/web-extension/src/storage.rs
    pub async fn remove_storage_value(key: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
    export async function get_storage_value(key) {
        try {
            let result = await chrome.storage.sync.get(key);
            if (result[key]) {
                return `${result[key]}`;
            }
            return "";
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    // need to rewrite with this: https://github.com/Pauan/tab-organizer/blob/rust/web-extension/src/storage.rs
    pub async fn get_storage_value(key: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
    export async function set_storage_value(key, value) {
        try {
            await chrome.storage.sync.set({ [key]: value });
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    // need to rewrite with this: https://github.com/Pauan/tab-organizer/blob/rust/web-extension/src/storage.rs
    pub async fn set_storage_value(key: &str, value: JsValue) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
    export function storageOnChange(callback) {
        chrome.storage.sync.onChanged.addListener((changes, namespace) => {
            Object.keys(changes).forEach((key) => {
                callback( { [key]: changes[key].newValue } );
            });
        });
    }
"#)]
extern "C" {
    pub fn storageOnChange(callback: &Closure<dyn Fn(JsValue)>);
}

/// Makes JS `console.log` available in Rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}
