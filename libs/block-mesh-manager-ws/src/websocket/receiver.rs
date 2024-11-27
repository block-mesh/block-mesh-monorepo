use crate::state::WsAppState;
use crate::websocket::process_message::process_message;
use axum::extract::ws::WebSocket;
use block_mesh_common::interfaces::ws_api::WsClientMessage;
use futures::stream::SplitStream;
use futures::StreamExt;
use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

pub async fn receiver(
    mut ws_stream: SplitStream<WebSocket>,
    is_cls: Arc<AtomicBool>,
    ip: String,
    task_scheduler_notifier: Arc<Notify>,
    state: Arc<WsAppState>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_stream.next().await {
            match process_message(msg.clone(), ip.clone(), state.clone()).await {
                ControlFlow::Continue(ws_client_message) => {
                    if let Some(ws_client_message) = ws_client_message {
                        if matches!(ws_client_message, WsClientMessage::CompleteTask(_)) {
                            task_scheduler_notifier.notify_waiters();
                        }
                    }
                }
                ControlFlow::Break(_) => {
                    tracing::trace!("Unhandled message: {msg:?}");
                    is_cls.store(true, Ordering::Relaxed);
                    return;
                }
            }
        }
    })
}
