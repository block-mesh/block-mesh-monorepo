use crate::database::task::update_task_assigned::update_task_assigned;
use crate::domain::task::TaskStatus;
use crate::startup::application::AppState;
use crate::ws::connection_manager::ConnectionManager;
use crate::ws::process_message::process_message;
use crate::ws::task_scheduler::TaskScheduler;
use aws_sdk_sesv2::config::IntoShared;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::{WsMessage, WsMessageTypes};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use leptos::ev::message;
use redis::transaction;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    state: Arc<AppState>,
    email: String,
) {
    let ws_connection_manager = state.ws_connection_manager.clone();
    let task_scheduler = ws_connection_manager.task_scheduler.clone();
    let broadcaster = ws_connection_manager.broadcaster.clone();
    let user_id = Uuid::new_v4();
    let mut broadcast_receiver = broadcaster.subscribe(user_id.clone()); // FIXME

    tokio::task::spawn(async move {
        while let Ok(msg) = broadcast_receiver.recv().await {
            // handle broadcast
        }
    });
    for i in 0..10 {
        task_scheduler
            .add_task(GetTaskResponse {
                id: Uuid::new_v4(),
                url: String::from(format!("https://example.com?={i}")),
                method: String::from("GET"),
                headers: None,
                body: None,
            })
            .await;
    }

    let org_email = Arc::new(email);
    let notify = Arc::new(Notify::new());
    let notify_r = notify.clone();
    let (mut sender, mut receiver) = socket.split();

    let pool = state.pool.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::trace!("WS Text: {text}");
                    notify_r.notify_one(); // only for demo
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_message) => match ws_message.message {
                            WsMessageTypes::SendTaskFromServer(_) => {}
                            WsMessageTypes::SubmitTaskToServer(task) => {
                                let mut transaction = pool.begin().await.unwrap();
                                update_task_assigned(
                                    &mut transaction,
                                    task.task_id,
                                    Uuid::new_v4(),
                                    TaskStatus::Completed,
                                )
                                .await
                                .unwrap();
                                transaction.commit().await.unwrap();
                                notify_r.notify_one();
                            }
                            WsMessageTypes::SendBandwidthReportFromServer => {}
                            WsMessageTypes::SubmitForBandwidthReportToServer(_) => {}
                            WsMessageTypes::SendUptimeFromServer => {}
                            WsMessageTypes::SubmitUptimeToServer(_) => {}
                        },
                        Err(_) => {
                            tracing::error!("Unsupported WS message");
                        }
                    }
                }
                Message::Binary(_) => {}
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Close(_) => {}
            }
        }
    });
    let send_task = tokio::spawn(async move {
        loop {
            let task_receiver = task_scheduler.add_session().await;
            let task = task_receiver.await.unwrap(); // waits for new task
            let ws_message = WsMessage {
                message_id: Uuid::new_v4(),
                email: Some(org_email.to_string()),
                device: None,
                message: WsMessageTypes::SendTaskFromServer(task),
            };
            // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            sender
                .send(Message::Text(serde_json::to_string(&ws_message).unwrap()))
                .await
                .unwrap();
            notify.notified().await;
        }
    });

    tokio::select! {
        o = recv_task => tracing::error!("recv_task task dead {:?}", o),
        o = send_task => tracing::error!("send_task dead {:?}", o)
    }

    broadcaster.unsubscribe(&user_id);
    tracing::info!("Websocket context {who} destroyed");
}

struct Messager(Arc<Mutex<SplitSink<WebSocket, WsMessage>>>);

impl Messager {
    fn new(sender: SplitSink<WebSocket, WsMessage>) -> Self {
        Self(Arc::new(Mutex::new(sender)))
    }

    // fn send(&self) {
    //     let mut guard = self.0.lock().unwrap().deref_mut();
    //     guard.send()
    //     // forbidden FIXME, read more on pinning
    // }
}
