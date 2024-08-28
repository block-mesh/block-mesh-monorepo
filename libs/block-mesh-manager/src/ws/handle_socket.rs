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
use uuid::Uuid;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    state: Arc<AppState>,
    email: String,
    connection_manager: ConnectionManager,
) {
    let task_manager = Arc::new(TaskManager::<GetTaskResponse>::new());
    for i in 0..10 {
        task_manager
            .add_task(GetTaskResponse {
                id: Uuid::new_v4(),
                url: String::from("https://example.com"),
                method: String::from("GET"),
                headers: None,
                body: None,
            })
            .await;
    }

    let org_email = Arc::new(email);

    let (mut sender, mut receiver) = socket.split();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let ws_message: WsMessage = serde_json::from_str(&text).unwrap();
                    match ws_message.message {
                        WsMessageTypes::SendTaskFromServer(_) => {}
                        WsMessageTypes::SubmitTaskToServer(task) => {
                            // TODO: Register this task in DB as Completed
                        }
                        WsMessageTypes::SendBandwidthReportFromServer => {}
                        WsMessageTypes::SubmitForBandwidthReportToServer(_) => {}
                        WsMessageTypes::SendUptimeFromServer => {}
                        WsMessageTypes::SubmitUptimeToServer(_) => {}
                    }
                }
                Message::Binary(_) => {}
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Close(_) => {}
            }
        }
    });
    loop {
        let task_receiver = task_manager.add_session().await;
        let task = task_receiver.await.unwrap(); // waits for new task
        let ws_message = WsMessage {
            message_id: Uuid::new_v4(),
            email: Some(org_email.to_string()),
            device: None,
            message: WsMessageTypes::SendTaskFromServer(task),
        };
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        sender
            .send(Message::Text(serde_json::to_string(&ws_message).unwrap()))
            .await
            .unwrap();
    }

    // let email = org_email.clone();
    // // This second task will receive messages from client and print them on server console
    // let tx_ws = state.tx_ws.clone();
    // let mut recv_task = tokio::spawn(async move {
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         tracing::info!("RECEIVER msg => {:#?}", msg);
    //         // print message and break if instructed to do so
    //         if process_message(&msg, who).is_break() {
    //             break;
    //         }
    //         match msg {
    //             Message::Text(text) => {
    //                 if let Ok(message) = serde_json::from_str::<WsMessage>(&text) {
    //                     if message.email.is_some() && message.email.clone().unwrap() == *email {
    //                         let _ = tx_ws.send(message);
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // });
    //
    // let email = org_email.clone();
    // // When the master send a message, the channel pass it along to the WS sender
    // let mut rx = state.rx_ws.resubscribe();
    // let rx_task = tokio::spawn(async move {
    //     while let Ok(msg) = rx.recv().await {
    //         if msg.email.is_some() && msg.email.clone().unwrap() == *email {
    //             if let Err(err) = sender
    //                 .send(Message::Text(serde_json::to_string(&msg).unwrap()))
    //                 .await
    //             {
    //                 tracing::error!("Couldn't send message to node");
    //             }
    //         }
    //     }
    // });

    // tokio::select! {
    //     o = recv_task => tracing::error!("recv_task task dead {:?}", o),
    //     o = rx_task => tracing::error!("rx_task dead {:?}", o)
    // }
    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed");
}
