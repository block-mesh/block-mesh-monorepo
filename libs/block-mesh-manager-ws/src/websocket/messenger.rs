use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use futures::stream::SplitSink;
use futures::SinkExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;

pub fn messenger(
    mut ws_sink: SplitSink<WebSocket, Message>,
    is_cls: Arc<AtomicBool>,
) -> (JoinHandle<()>, tokio::sync::mpsc::Sender<WsServerMessage>) {
    let (sink_tx, mut sink_rx) = tokio::sync::mpsc::channel::<WsServerMessage>(10);
    let sink_task = tokio::spawn(async move {
        // Any message coming from the sync channel rx, is sent to ws tx/sink
        while let Some(server_message) = sink_rx.recv().await {
            if let Ok(serialized_server_message) = serde_json::to_string(&server_message) {
                if let Err(error) = ws_sink.send(Message::Text(serialized_server_message)).await {
                    if is_cls.load(Ordering::Relaxed) {
                        return;
                    }
                    tracing::error!("Sink task error: {error}");
                }
            } else {
                tracing::error!("Failed to serialize message {server_message:?}");
            }
        }
    });
    (sink_task, sink_tx)
}
