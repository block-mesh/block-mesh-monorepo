use block_mesh_common::{
    constants::DeviceType,
    interfaces::{
        server_api::ReportUptimeRequest,
        ws_api::{WsMessage, WsMessageTypes},
    },
};
use leptos::SignalGetUntracked;
use speed_test::metadata::fetch_metadata_blocking;
use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent};

use crate::utils::extension_wrapper_state::ExtensionWrapperState;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn on_message_handler(
    ws: web_sys::WebSocket,
    app_state: ExtensionWrapperState,
) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let email = app_state.email.get_untracked();
        let api_token = app_state.api_token.get_untracked();
        let metadata = fetch_metadata_blocking().unwrap_or_default();
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            match serde_json::from_str::<WsMessage>(
                &txt.as_string()
                    .unwrap_or("Couldn't covert Js String to String".to_string()),
            ) {
                Ok(message) => match message.message {
                    WsMessageTypes::SendUptimeFromServer => {
                        console_log!("Server asked for an uptime request");
                        let query = ReportUptimeRequest {
                            email: email.clone(),
                            api_token,
                            ip: if metadata.ip.is_empty() {
                                None
                            } else {
                                Some(metadata.ip)
                            },
                        };
                        let submit_uptime = WsMessageTypes::SubmitUptimeToServer(query);
                        let ws_message = WsMessage {
                            device: Some(DeviceType::Extension),
                            email: Some(email),
                            message: submit_uptime,
                            message_id: uuid::Uuid::new_v4(),
                        };
                        match ws.send_with_str(
                            &serde_json::to_string::<WsMessage>(&ws_message).unwrap(),
                        ) {
                            Err(err) => console_log!("Couldn't send back uptime. {:#?} \n", err),
                            Ok(_) => console_log!("Sent uptime to server successfully.\n"),
                        }
                    }
                    _ => {
                        console_log!("Recieved Message from server : {:#?}", message);
                    }
                },
                Err(err) => console_log!("Error parsing message {err}"),
            }
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    })
}

pub fn on_error_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut(ErrorEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    })
}

pub fn on_open_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
        match ws.send_with_str("ping") {
            Ok(_) => console_log!("Sent a ping message."),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    })
}
