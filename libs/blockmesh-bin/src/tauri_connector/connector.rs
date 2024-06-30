use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::interfaces::server_api::{LoginForm, RegisterForm};
use leptos::tracing;
use serde::{Deserialize, Serialize};
use solana_sdk::wasm_bindgen;
use std::fmt::Display;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
        export async function invoke(cmd, args) {
            try {
                return await window.__TAURI__.tauri.invoke(cmd, args);
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

pub async fn invoke_tauri(cmd: &str, args: JsValue) -> Result<JsValue, MyJsError> {
    tracing::info!("here 1");
    let result = invoke(cmd, args).await;
    tracing::info!("here 2");
    let error_attribute = JsValue::from_str("error");
    if let Ok(error) = js_sys::Reflect::get(&result, &error_attribute) {
        if error.is_string() {
            let error = error.as_string().unwrap();
            tracing::error!("Command: '{}' , Failed with error: '{}'", cmd, error);
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
