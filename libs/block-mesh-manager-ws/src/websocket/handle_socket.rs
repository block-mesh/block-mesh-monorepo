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
pub async fn handle_socket(
    email: String,
    socket: WebSocket,
    ip: String,
    state: Arc<AppState>,
    user_id: Uuid,
) {
    let is_closing = Arc::new(AtomicBool::new(false));
    let (ws_sink, ws_stream) = socket.split();
    let (sink_task, sink_tx) = messenger(ws_sink, is_closing.clone());
    // Using notify to process one task at a time
    let task_scheduler_notifier = Arc::new(Notify::new());
    let recv_task = receiver(
        ws_stream,
        is_closing.clone(),
        ip.clone(),
        task_scheduler_notifier.clone(),
        state.clone(),
    )
    .await;

    let ws_connection_manager = state.websocket_manager.clone();
    let task_scheduler = ws_connection_manager.task_scheduler.clone();
    let broadcaster = ws_connection_manager.broadcaster.clone();
    let mut broadcast_receiver = broadcaster
        .subscribe(email.clone(), user_id, ip.clone(), sink_tx.clone())
        .await;
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
            let _ = match task_receiver.await {
                Ok(task) => task,
                Err(_) => {
                    tracing::trace!("Task scheduler was dropper");
                    return;
                }
            };
            task_scheduler_notifier.notified().await; // wait for task to complete on the client side
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
                tracing::trace!("Failed to pass a message to task_sink_tx");
            }
        }
    });

    tokio::select! {
        o = recv_task => tracing::trace!("recv_task dead {:?}", o),
        o = send_task => tracing::trace!("send_task dead {:?}", o),
        o = sink_task => tracing::trace!("sink_task dead {:?}", o),
        o = broadcast_task => tracing::trace!("broadcast_task dead {:?}", o)
    }

    broadcaster.unsubscribe(email, user_id, ip.clone()).await;
    tracing::trace!("Websocket context destroyed");
}
