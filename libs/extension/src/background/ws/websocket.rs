use super::{
    on_close_handler, on_error_handler, on_message_handler, on_open_handler, WebSocketReadyState,
};
use crate::background::bandwidth_measurement::measure_bandwidth_inner;
use crate::background::tasks_manager::task_poller_inner;
use crate::background::uptime_reporter::report_uptime_inner;
use crate::utils::log::log;
use crate::utils::{connectors::set_panic_hook, extension_wrapper_state::ExtensionWrapperState};
use block_mesh_common::chrome_storage::AuthStatus;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use flume::{Receiver, Sender};
use leptos::{spawn_local, SignalGetUntracked};
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

pub static WEB_SOCKET_STATUS: OnceCell<Arc<Mutex<WebSocketReadyState>>> = OnceCell::new();
pub static RX: OnceCell<Arc<Receiver<WsServerMessage>>> = OnceCell::new();
pub static TX: OnceCell<Arc<Sender<WsServerMessage>>> = OnceCell::new();

pub fn get_ws_status() -> WebSocketReadyState {
    let ws_status =
        WEB_SOCKET_STATUS.get_or_init(|| Arc::new(Mutex::new(WebSocketReadyState::CLOSED)));
    ws_status.lock().unwrap().clone()
}

pub fn set_ws_status(status: &WebSocketReadyState) {
    let ws_status =
        WEB_SOCKET_STATUS.get_or_init(|| Arc::new(Mutex::new(WebSocketReadyState::CLOSED)));
    *ws_status.lock().unwrap() = status.clone();
}

pub fn set_rx(rx: Receiver<WsServerMessage>) {
    match RX.get() {
        Some(_) => {}
        None => {
            let r = RX.get_or_init(|| Arc::new(rx));
            spawn_local(async move {
                while let Ok(msg) = r.recv_async().await {
                    let app_state = ExtensionWrapperState::default();
                    app_state.init_with_storage().await;
                    log!("RX msg {:?} - {:?}", msg, app_state);

                    if !app_state.has_api_token() {
                        continue;
                    }
                    if app_state.status.get_untracked() == AuthStatus::LoggedOut {
                        continue;
                    }
                    let base_url = app_state.blockmesh_url.get_untracked();
                    let email = app_state.email.get_untracked();
                    let api_token = app_state.api_token.get_untracked();

                    match msg {
                        WsServerMessage::RequestUptimeReport => {
                            report_uptime_inner(&base_url, &email, &api_token).await;
                        }
                        WsServerMessage::RequestBandwidthReport => {
                            measure_bandwidth_inner(&base_url, &email, &api_token).await;
                        }
                        WsServerMessage::AssignTask(task) => {
                            task_poller_inner(&base_url, &email, &api_token, &task).await;
                        }
                    }
                }
            });
        }
    }
}

pub fn set_tx(tx: Sender<WsServerMessage>) {
    TX.get_or_init(|| Arc::new(tx));
}

pub fn get_tx() -> Option<Arc<Sender<WsServerMessage>>> {
    TX.get().cloned()
}

#[wasm_bindgen]
pub async fn start_websocket() -> Result<(), JsValue> {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    log!("start_websocket");
    let (tx, rx) = flume::unbounded::<WsServerMessage>();
    set_tx(tx);
    set_rx(rx);
    let app_state = ExtensionWrapperState::default();
    app_state.init_with_storage().await;
    if !app_state.has_api_token() {
        return Err(JsValue::from_str("Missing Api Token"));
    }
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let blockmesh_url = app_state
        .blockmesh_url
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

    log!("connecting websocket ws://{blockmesh_url}/ws?email={email}&api_token={api_token}");
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
