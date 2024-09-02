use crate::startup::application::AppState;
use crate::ws::process_message::process_message;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::ws_api::{WsClientMessage, WsServerMessage};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
fn messenger(
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

fn receiver(
    mut ws_stream: SplitStream<WebSocket>,
    is_cls: Arc<AtomicBool>,
    who: SocketAddr,
    task_scheduler_notifier: Arc<Notify>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_stream.next().await {
            match process_message(msg, who) {
                ControlFlow::Continue(ws_client_message) => {
                    if let Some(ws_client_message) = ws_client_message {
                        if matches!(ws_client_message, WsClientMessage::CompleteTask(_)) {
                            task_scheduler_notifier.notify_one();
                        }
                    }
                }
                ControlFlow::Break(_) => {
                    is_cls.store(true, Ordering::Relaxed);
                    return;
                }
            }
        }
    })
}

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    state: Arc<AppState>,
    _email: String,
    user_id: Uuid,
) {
    let is_closing = Arc::new(AtomicBool::new(false));
    let (ws_sink, ws_stream) = socket.split();
    let (sink_task, sink_tx) = messenger(ws_sink, is_closing.clone());
    let notify = Arc::new(Notify::new());
    let recv_task = receiver(ws_stream, is_closing.clone(), who, notify.clone());

    let ws_connection_manager = state.ws_connection_manager.clone();
    let task_scheduler = ws_connection_manager.task_scheduler;
    let broadcaster = ws_connection_manager.broadcaster;

    let mut broadcast_receiver = broadcaster.subscribe(user_id, sink_tx.clone()); // FIXME

    // demo
    // FIXME
    broadcaster
        .batch(WsServerMessage::RequestUptimeReport, &[user_id])
        .await;
    // for _i in 0..10 {
    //     task_scheduler
    //         .add_task(GetTaskResponse {
    //             id: Uuid::new_v4(),
    //             url: String::from("https://example.com"),
    //             headers: None,
    //             body: None,
    //             method: String::from("GET"),
    //         })
    //         .await;
    // }

    // Using notify to process one task at a time
    let notify = Arc::new(Notify::new());
    let _task_sink_tx = sink_tx.clone();
    let is_cls = is_closing.clone();
    let send_task = tokio::spawn(async move {
        loop {
            let Some(task_receiver) = task_scheduler.add_session().await else {
                if is_closing.load(Ordering::Relaxed) {
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
            }; // waits for new task
               // FIXME
               // if let Err(_error) = _task_sink_tx.send(WsServerMessage::AssignTask(_task)).await {
               //     if is_cls.load(Ordering::Relaxed) {
               //         return;
               //     }
               //     tracing::error!("Task scheduler was dropped");
               // }
            notify.notified().await; // wait for task to complete on the client side
        }
    });

    let is_cls = is_closing.clone();
    let task_sink_tx = sink_tx.clone();
    let broadcast_task = tokio::spawn(async move {
        while let Ok(broadcast_message) = broadcast_receiver.recv().await {
            tracing::info!("Broadcast received {broadcast_message:?}");
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

    broadcaster.unsubscribe(&user_id);
    tracing::info!("Websocket context {who} destroyed");
}
