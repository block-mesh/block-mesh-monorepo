use crate::startup::application::AppState;
use crate::ws::connection_manager::ConnectionManager;
use crate::ws::process_message::process_message;
use crate::ws::task_manager::TaskManager;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::{WsMessage, WsMessageTypes};
use futures::{SinkExt, StreamExt};
use leptos::ev::message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Notify;
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    state: Arc<AppState>,
    email: String,
    task_manager: Arc<TaskManager<GetTaskResponse>>,
) {
    for i in 0..10 {
        task_manager
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
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::trace!("WS Text: {text}");
                    notify_r.notify_one(); // only for demo
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_message) => {
                            match ws_message.message {
                                WsMessageTypes::SendTaskFromServer(_) => {}
                                WsMessageTypes::SubmitTaskToServer(task) => {
                                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                    // TODO: Register this task in DB as Completed
                                    notify_r.notify_one();
                                }
                                WsMessageTypes::SendBandwidthReportFromServer => {}
                                WsMessageTypes::SubmitForBandwidthReportToServer(_) => {}
                                WsMessageTypes::SendUptimeFromServer => {}
                                WsMessageTypes::SubmitUptimeToServer(_) => {}
                            }
                        }
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
            let task_receiver = task_manager.add_session().await;
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

    tracing::info!("Websocket context {who} destroyed");
}
