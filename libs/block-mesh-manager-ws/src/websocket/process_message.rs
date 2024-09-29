use crate::state::AppState;
use axum::extract::ws::Message;
use block_mesh_common::interfaces::ws_api::WsClientMessage;
use std::ops::ControlFlow;

pub async fn process_message(
    msg: Message,
    state: AppState,
) -> ControlFlow<(), Option<WsClientMessage>> {
    match msg {
        Message::Text(text) => {
            let ws_client_message = process_client_message(&text, state).await;
            return ControlFlow::Continue(ws_client_message);
        }
        Message::Binary(bytes) => {
            tracing::info!(">>> sent {} bytes: {:?}", bytes.len(), bytes);
        }
        Message::Close(frame) => {
            if let Some(cf) = frame {
                tracing::info!(
                    ">>> sent close with code {} and reason `{}`",
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> somone somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(bytes) => {
            tracing::info!(">>> sent pong with {bytes:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(bytes) => {
            tracing::info!(">>> sent ping with {bytes:?}");
        }
    }
    ControlFlow::Continue(None)
}

async fn process_client_message(text: &str, _state: AppState) -> Option<WsClientMessage> {
    match serde_json::from_str::<WsClientMessage>(text) {
        Ok(message) => {
            match &message {
                WsClientMessage::CompleteTask(_query) => {}
                WsClientMessage::ReportBandwidth(_body) => {}
                WsClientMessage::ReportUptime(_query) => {}
                WsClientMessage::Ping => {}
            }
            return Some(message);
        }
        Err(_) => {
            tracing::info!("Invalid Message => {}", text)
        }
    }
    None
}
