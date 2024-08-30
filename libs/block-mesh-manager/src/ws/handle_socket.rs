use crate::database::task::update_task_assigned::update_task_assigned;
use crate::domain::task::TaskStatus;
use crate::startup::application::AppState;
use crate::ws::connection_manager::ConnectionManager;
use crate::ws::process_message::process_message;
use crate::ws::task_scheduler::TaskScheduler;
use aws_sdk_sesv2::config::IntoShared;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::{
    WsClientMessage, WsMessage, WsMessageTypes, WsServerMessage,
};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use leptos::ev::message;
use redis::transaction;
use std::net::SocketAddr;
use std::ops::{ControlFlow, Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::Notify;
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    state: Arc<AppState>,
    email: String,
    user_id: Uuid,
) {
    let is_closing = Arc::new(AtomicBool::new(false));
    let (mut ws_sink, mut ws_stream) = socket.split();
    let (sink_tx, mut sink_rx) = tokio::sync::mpsc::channel::<WsServerMessage>(10);

    let is_cls = is_closing.clone();
    let sink_task = tokio::spawn(async move {
        while let Some(server_message) = sink_rx.recv().await {
            let Ok(serialized_server_message) = serde_json::to_string(&server_message) else {
                tracing::error!("Failed to serialize message {server_message:?}");
                continue;
            };

            if let Err(error) = ws_sink.send(Message::Text(serialized_server_message)).await {
                if is_cls.load(Ordering::Relaxed) {
                    return;
                }
                tracing::error!("Sink task error: {error}");
            }
        }
    });

    let ws_connection_manager = state.ws_connection_manager.clone();
    let task_scheduler = ws_connection_manager.task_scheduler;
    let broadcaster = ws_connection_manager.broadcaster;

    let mut broadcast_receiver = broadcaster.subscribe(user_id.clone(), sink_tx.clone()); // FIXME

    // demo
    broadcaster
        .batch(WsServerMessage::RequestUptimeReport, &[user_id.clone()])
        .await;
    for i in 0..10 {
        task_scheduler
            .add_task(GetTaskResponse {
                id: Uuid::new_v4(),
                url: String::from("https://example.com"),
                headers: None,
                body: None,
                method: String::from("GET"),
            })
            .await;
    }

    let notify = Arc::new(Notify::new());

    let is_cls = is_closing.clone();
    let task_scheduler_notifier = notify.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_stream.next().await {
            let control_flow = process_message(msg, who, task_scheduler_notifier.clone());
            match control_flow {
                ControlFlow::Continue(_) => {}
                ControlFlow::Break(_) => {
                    is_cls.store(true, Ordering::Relaxed);
                    return;
                }
            }
        }
    });

    let task_sink_tx = sink_tx.clone();
    let is_cls = is_closing.clone();
    let send_task = tokio::spawn(async move {
        loop {
            let Some(task_receiver) = task_scheduler.add_session().await else {
                if is_cls.load(Ordering::Relaxed) {
                    return;
                }
                continue;
            };
            let task = match task_receiver.await {
                Ok(task) => task,
                Err(_) => {
                    tracing::info!("Task scheduler was dropper");
                    return;
                }
            }; // waits for new task
            if let Err(error) = task_sink_tx.send(WsServerMessage::AsignTask(task)).await {
                if is_cls.load(Ordering::Relaxed) {
                    return;
                }
                tracing::error!("Task scheduler was dropped");
            }
            notify.notified().await; // wait for task to complete on the client side
        }
    });

    let is_cls = is_closing.clone();
    let task_sink_tx = sink_tx.clone();
    let broadcast_task = tokio::spawn(async move {
        while let Ok(broadcast_message) = broadcast_receiver.recv().await {
            tracing::info!("Broadcast received {broadcast_message:?}");
            if let Err(error) = task_sink_tx.send(broadcast_message).await {
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
