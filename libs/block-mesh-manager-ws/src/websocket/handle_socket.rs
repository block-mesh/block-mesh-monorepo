use crate::state::AppState;
use crate::websocket::messenger::messenger;
use crate::websocket::receiver::receiver;
use axum::extract::ws::WebSocket;
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(socket: WebSocket, ip: String, state: Arc<AppState>, user_id: Uuid) {
    let is_closing = Arc::new(AtomicBool::new(false));
    let (ws_sink, ws_stream) = socket.split();
    let (sink_task, sink_tx) = messenger(ws_sink, is_closing.clone());
    let notify = Arc::new(Notify::new());
    let recv_task = receiver(
        ws_stream,
        is_closing.clone(),
        ip.clone(),
        notify.clone(),
        state.clone(),
    )
    .await;

    let ws_connection_manager = state.websocket_manager.clone();
    let task_scheduler = ws_connection_manager.task_scheduler;
    let broadcaster = ws_connection_manager.broadcaster;
    let mut broadcast_receiver = broadcaster
        .subscribe(user_id, ip.clone(), sink_tx.clone())
        .await;
    // Using notify to process one task at a time
    let notify = Arc::new(Notify::new());
    let _task_sink_tx = sink_tx.clone();
    let is_cls = is_closing.clone();
    let send_task = tokio::spawn(async move {
        loop {
            let Some(task_receiver) = task_scheduler.add_session().await else {
                if is_cls.load(Ordering::Relaxed) {
                    return;
                }
                continue;
            };
            let _task = match task_receiver.await {
                Ok(task) => task,
                Err(_) => {
                    tracing::info!("Task scheduler was dropper");
                    return;
                }
            };
            notify.notified().await; // wait for task to complete on the client side
        }
    });

    let is_cls = is_closing.clone();
    let task_sink_tx = sink_tx.clone();
    let broadcast_task = tokio::spawn(async move {
        while let Ok(broadcast_message) = broadcast_receiver.recv().await {
            if let Err(_error) = task_sink_tx.send(broadcast_message).await {
                if is_cls.load(Ordering::Relaxed) {
                    return;
                }
                tracing::error!("Failed to pass a message to task_sink_tx");
            }
        }
    });

    tokio::select! {
        o = recv_task => tracing::error!("recv_task dead {:?}", o),
        o = send_task => tracing::error!("send_task dead {:?}", o),
        o = sink_task => tracing::error!("sink_task dead {:?}", o),
        o = broadcast_task => tracing::error!("broadcast_task dead {:?}", o)
    }

    broadcaster.unsubscribe(user_id, ip.clone()).await;
    tracing::info!("Websocket context destroyed");
}
