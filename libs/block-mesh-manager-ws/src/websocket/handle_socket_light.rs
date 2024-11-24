use crate::state::AppState;
use crate::websocket::process_message_light::process_message_light;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::db_messages::{
    AggregateAddToMessage, DBMessage, DBMessageTypes,
};
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use futures::{SinkExt, StreamExt};
use sqlx::types::chrono::Utc;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub async fn handle_socket_light(
    email: String,
    mut socket: WebSocket,
    ip: String,
    state: Arc<AppState>,
    user_id: Uuid,
) {
    let sleep = env::var("WS_KEEP_ALIVE")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(15000);
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        tracing::trace!("Pinged {ip}...");
    } else {
        tracing::trace!("Could not send ping {ip}!");
        return;
    }

    let ws_connection_manager = state.websocket_manager.clone();
    let broadcaster = ws_connection_manager.broadcaster.clone();
    let mut redis = state.redis.clone();
    broadcaster
        .subscribe_light(&email, &user_id, &mut redis)
        .await;
    let (mut sender, mut receiver) = socket.split();
    let tx = state.tx.clone();
    let mut send_task = tokio::spawn(async move {
        let mut prev = Utc::now();
        // Send to client - keep alive via ping
        loop {
            let _ = sender.send(Message::Ping(vec![1, 2, 3])).await;
            tokio::time::sleep(Duration::from_millis(sleep)).await;
            let now = Utc::now();
            let delta = (now - prev).num_seconds();
            let _ = tx
                .send_async(DBMessage::AggregateAddToMessage(AggregateAddToMessage {
                    msg_type: DBMessageTypes::AggregateAddToMessage,
                    user_id,
                    value: serde_json::Value::from(delta),
                    name: AggregateName::Uptime.to_string(),
                }))
                .await;
            prev = Utc::now();
        }
    });

    let ip_c = ip.clone();
    let mut recv_task = tokio::spawn(async move {
        // Receive from client
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message_light(msg, &ip_c).is_break() {
                break;
            }
        }
    });

    tokio::select! {
        rv_a = &mut send_task => {
            match rv_a {
                Ok(_) => tracing::trace!("send_task done {ip}"),
                Err(e) => tracing::trace!("send_task error {e}")
            }
            recv_task.abort();
        },
        rv_b = &mut recv_task => {
            match rv_b {
                Ok(_) => tracing::trace!("recv_task done {ip}"),
                Err(e) => tracing::trace!("recv_task error {e}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    let mut redis = state.redis.clone();
    broadcaster
        .unsubscribe_light(&email, &user_id, &mut redis)
        .await;
    tracing::trace!("Websocket context {ip} destroyed");
}
