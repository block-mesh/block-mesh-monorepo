use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::chrome_storage::{MessageKey, MessageType, MessageValue, PostMessage};
use block_mesh_common::interfaces::server_api::{LoginForm, RegisterForm};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
    export function onPostMessage(callback) {
        if (!window.message_channel_port) return;
            window.message_channel_port.addEventListener("message", (msg) => {
                console.log("tauri connector.rs event listener", {msg});
                const {data} = msg;
                callback(data);
            });
        }"#)]
extern "C" {
    pub fn onPostMessage(callback: &Closure<dyn Fn(JsValue)>);
}

#[wasm_bindgen(inline_js = r#"
        export async function invoke(cmd, args) {
            try {
                return await window.__TAURI__.core.invoke(cmd, args);
            } catch (e) {
                console.error(`Error in invoke ${cmd} : ${e}`);
                const t = typeof e;
                if (t === 'string') {
                    return { error: e };
                } else if (t === 'object') {
                    return { error: e?.message };
                } else {
                    return { error: 'Unknown error', e };
                }
            }
        }"#)]
extern "C" {
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
    export async function send_message(msg) {
        try {
            if (!window.message_channel_port) {
                console.log("blockmesh-bin message_channel_port is missing", window.mounted, window.message_channel_port);
                return;
            }
            window.message_channel_port.postMessage(msg);
        } catch (e) {
            return ""
        }
    }"#)]
extern "C" {
    pub async fn send_message(msg: JsValue) -> JsValue;
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

pub async fn invoke_tauri(cmd: &str, args: JsValue) -> Result<JsValue, MyJsError> {
    let result = invoke(cmd, args).await;
    let error_attribute = JsValue::from_str("error");
    if let Ok(error) = js_sys::Reflect::get(&result, &error_attribute) {
        if error.is_string() {
            let error = error.as_string().unwrap();
            return Err(MyJsError {
                message: error,
                cmd: cmd.to_string(),
            });
        }
    }
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyJsError {
    pub cmd: String,
    pub message: String,
}

impl Display for MyJsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command '{}': | Error: '{}'", self.cmd, self.message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAppConfigArgs {
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToggleMinerArgs {
    pub task_status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginArgs {
    pub login_form: LoginForm,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterArgs {
    pub register_form: RegisterForm,
}
