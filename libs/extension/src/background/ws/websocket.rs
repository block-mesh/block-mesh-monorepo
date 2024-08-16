use crate::utils::{connectors::set_panic_hook, extension_wrapper_state::ExtensionWrapperState};

use super::{on_error_handler, on_message_handler, on_open_handler};
use block_mesh_common::constants::DeviceType;
use leptos::SignalGetUntracked;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

#[wasm_bindgen]
pub async fn start_websocket() -> Result<(), JsValue> {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = ExtensionWrapperState::default();
    app_state.init_with_storage().await;

    if !app_state.has_api_token() {
        return Err(JsValue::from_str("Missing Api Token"));
    }
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let ws = WebSocket::new(&format!(
        "ws://127.0.0.1:8000/ws?email={email}&api_token={api_token}"
    ))?;

    let onopen_callback = on_open_handler(ws.clone());
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onmessage_callback = on_message_handler(ws.clone(), app_state);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = on_error_handler(ws.clone());
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    Ok(())
}
