use super::{
    on_close_handler, on_error_handler, on_message_handler, on_open_handler, WebSocketReadyState,
};
use crate::background::ws::channel::get_tx;
use crate::utils::log::log;
use crate::utils::{connectors::set_panic_hook, extension_wrapper_state::ExtensionWrapperState};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use leptos::SignalGetUntracked;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

pub static WEB_SOCKET_STATUS: OnceCell<Arc<RwLock<WebSocketReadyState>>> = OnceCell::new();

pub fn get_ws_status() -> WebSocketReadyState {
    let ws_status =
        WEB_SOCKET_STATUS.get_or_init(|| Arc::new(RwLock::new(WebSocketReadyState::CLOSED)));
    ws_status.read().unwrap().clone()
}

pub fn set_ws_status(status: &WebSocketReadyState) {
    let ws_status =
        WEB_SOCKET_STATUS.get_or_init(|| Arc::new(RwLock::new(WebSocketReadyState::CLOSED)));
    *ws_status.write().unwrap() = status.clone();
}

#[wasm_bindgen]
pub async fn stop_websocket() {
    if let Some(tx) = get_tx() {
        let _ = tx.read().unwrap().send(WsServerMessage::CloseConnection);
    }
}

#[wasm_bindgen]
pub async fn start_websocket() -> Result<(), JsValue> {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    log!("start_websocket");

    let app_state = ExtensionWrapperState::default();
    app_state.init_with_storage().await;
    if !app_state.has_api_token() {
        return Err(JsValue::from_str("Missing Api Token"));
    }
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let blockmesh_url = app_state
        .blockmesh_ws_url
        .get_untracked()
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    let api_token = api_token.to_string();
    match get_ws_status() {
        WebSocketReadyState::CLOSED => {}
        WebSocketReadyState::CLOSING => {}
        WebSocketReadyState::OPEN => return Ok(()),
        WebSocketReadyState::CONNECTING => return Ok(()),
        WebSocketReadyState::INVALID => return Ok(()),
    }
    log!("connecting websocket {blockmesh_url}/ws?email={email}&api_token={api_token}");
    let ws = WebSocket::new(&format!(
        "{blockmesh_url}/ws?email={email}&api_token={api_token}"
    ))?;

    let state: WebSocketReadyState = ws.ready_state().into();
    set_ws_status(&state);

    let onopen_callback = on_open_handler(ws.clone());
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onmessage_callback = on_message_handler(ws.clone(), app_state);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = on_error_handler(ws.clone());
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onclose_callback = on_close_handler(ws.clone());
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();

    Ok(())
}
