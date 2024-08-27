use crate::startup::application::AppState;
use crate::ws::connection_manager::ConnectionManager;
use crate::ws::process_message::process_message;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::ws_api::WsMessage;
use futures::{SinkExt, StreamExt};
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
    let org_email = Arc::new(email);
    // let (tx, mut rx) = tokio::sync::mpsc::channel::<WsMessage>(50);
    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(&msg, who).is_break() {
                return;
            }
        } else {
            tracing::info!("client {who} abruptly disconnected");
            return;
        }
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();
    sender
        .send(Message::Text(String::from("Hello")))
        .await
        .unwrap();
    let email = org_email.clone();
    // This second task will receive messages from client and print them on server console
    let tx_ws = state.tx_ws.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            tracing::info!("RECEIVER msg => {:#?}", msg);
            // print message and break if instructed to do so
            if process_message(&msg, who).is_break() {
                break;
            }
            match msg {
                Message::Text(text) => {
                    if let Ok(message) = serde_json::from_str::<WsMessage>(&text) {
                        if message.email.is_some() && message.email.clone().unwrap() == *email {
                            let _ = tx_ws.send(message);
                        }
                    }
                }
                _ => {}
            }
        }
    });

    let email = org_email.clone();
    // When the master send a message, the channel pass it along to the WS sender
    let mut rx = state.rx_ws.resubscribe();
    let rx_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg.email.is_some() && msg.email.clone().unwrap() == *email {
                if let Err(err) = sender
                    .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                    .await
                {
                    tracing::error!("Couldn't send message to node");
                }
            }
        }
    });

    tokio::select! {
        o = recv_task => tracing::error!("recv_task task dead {:?}", o),
        o = rx_task => tracing::error!("rx_task dead {:?}", o)
    }
    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed");
}
