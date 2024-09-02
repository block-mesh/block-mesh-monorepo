use axum::extract::ws::Message;
use block_mesh_common::interfaces::ws_api::WsClientMessage;
use std::net::SocketAddr;
use std::ops::ControlFlow;

/// helper to print contents of messages to stdout. Has special treatment for Close.
pub fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), Option<WsClientMessage>> {
    tracing::info!("PROCESS_MESSAGE msg = {:#?}", msg);
    match msg {
        Message::Text(text) => {
            let ws_client_message = process_client_message(&text, who);
            return ControlFlow::Continue(ws_client_message);
        }
        Message::Binary(bytes) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, bytes.len(), bytes);
        }
        Message::Close(frame) => {
            if let Some(cf) = frame {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(bytes) => {
            tracing::info!(">>> {who} sent pong with {bytes:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(bytes) => {
            tracing::info!(">>> {who} sent ping with {bytes:?}");
        }
    }
    ControlFlow::Continue(None)
}

fn process_client_message(text: &str, _who: SocketAddr) -> Option<WsClientMessage> {
    match serde_json::from_str::<WsClientMessage>(text) {
        Ok(message) => {
            match message {
                WsClientMessage::CompleteTask(_task) => {
                    // TODO: Sync DB row
                }
                WsClientMessage::ReportBandwidth => {}
                WsClientMessage::ReportUptime => {}
            }
            return Some(message);
        }
        Err(_) => {
            tracing::info!("Invalid Message => {}", text)
        }
    }
    None
}
