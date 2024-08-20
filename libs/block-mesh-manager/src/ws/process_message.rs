use axum::extract::ws::Message;
use block_mesh_common::interfaces::ws_api::{WsMessage, WsMessageTypes};
use std::net::SocketAddr;
use std::ops::ControlFlow;

/// helper to print contents of messages to stdout. Has special treatment for Close.
pub fn process_message(msg: &Message, who: SocketAddr) -> ControlFlow<(), ()> {
    tracing::info!("PROCESS_MESSAGE msg = {:#?}", msg);
    match msg {
        Message::Text(t) => {
            handle_ws_message(t, who);
        }
        Message::Binary(d) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
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

        Message::Pong(v) => {
            tracing::info!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::info!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

fn handle_ws_message(s: &str, who: SocketAddr) {
    match serde_json::from_str::<WsMessage>(s) {
        Ok(message) => {
            let message_id = message.message_id;
            let device = message.device;
            let email = message.email;
            match message.message {
                WsMessageTypes::SubmitUptimeToServer(report) => {
                    // Whenever Client sends a message a receiver worker logs them already
                    // tracing::info!(
                    //     "Received Uptime Report from {}/{:?}: {:?}",
                    //     who,
                    //     email,
                    //     report
                    // );
                }
                _ => {}
            }
        }
        Err(_) => {
            tracing::info!("Invalid Message => {}", s)
        }
    }
}
