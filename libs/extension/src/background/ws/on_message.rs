use crate::background::ws::channel::{get_tx, setup_channels};
use crate::background::ws::websocket::set_ws_status;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use crate::utils::log::{log, log_error};
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use wasm_bindgen::prelude::*;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent};

pub fn on_message_handler(
    ws: web_sys::WebSocket,
    _app_state: ExtensionWrapperState,
) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        log!("on_message_handle => {:#?}", e);
        log!("on_message_handle e.data() => {:#?}", e.data());
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            if txt == "ping" {
                let _ = ws.send_with_str("pong");
            }
            match WsServerMessage::try_from(txt.as_string().unwrap_or_default()) {
                Ok(msg) => {
                    log!("on_message msg => {:#?}", msg);
                    if let Some(tx) = get_tx() {
                        if let Ok(tx) = tx.lock() {
                            let _ = tx.try_send(msg);
                        }
                    }
                }
                Err(error) => {
                    log_error!("on_message_handle js error => {:#?}", error);
                }
            }
        } else {
            log_error!("message event, received Unknown: {:?}", e.data());
        }
    })
}

pub fn on_error_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut(ErrorEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        let state: WebSocketReadyState = ws.ready_state().into();
        set_ws_status(&state);
        log_error!(
            "on_error_handler:: closing ws with error error event: {:?} | {:?}",
            e.error().as_string(),
            state
        );
    })
}

pub fn on_open_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || match ws.clone().send_with_str("ping") {
        Ok(_) => {
            log!("Sent a ping message.");
            setup_channels(ws.clone());
        }
        Err(err) => log_error!("error sending message: {:?}", err),
    })
}

pub fn on_close_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut(CloseEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: CloseEvent| {
        let state: WebSocketReadyState = ws.ready_state().into();
        set_ws_status(&state);
        log_error!(
            "on_close_handler:: closing ws with error error event: {:?} | {:?} | {:?}",
            e.code(),
            e.reason(),
            state
        );
    })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum WebSocketReadyState {
    CONNECTING,
    OPEN,
    CLOSING,
    CLOSED,
    INVALID,
}

impl From<u16> for WebSocketReadyState {
    fn from(value: u16) -> Self {
        match value {
            0 => WebSocketReadyState::CONNECTING,
            1 => WebSocketReadyState::OPEN,
            2 => WebSocketReadyState::CLOSING,
            3 => WebSocketReadyState::CLOSED,
            _ => WebSocketReadyState::INVALID,
        }
    }
}
