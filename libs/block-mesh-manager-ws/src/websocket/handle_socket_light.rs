use crate::state::WsAppState;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::db_messages::{
    AggregateAddToMessage, AggregateSetToMessage, CreateDailyStatMessage, DBMessage, DBMessageTypes,
};
use block_mesh_common::interfaces::ws_api::{WsClientMessage, WsServerMessage};
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
    state: Arc<WsAppState>,
    user_id: Uuid,
) {
    let counter_period = env::var("COUNTER_PERIOD")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(50u64);
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

    let tx_c = state.tx.clone();

    let mut create_daily_state_task = tokio::spawn(async move {
        loop {
            let _ = tx_c
                .send_async(DBMessage::CreateDailyStatMessage(CreateDailyStatMessage {
                    msg_type: DBMessageTypes::CreateDailyStatMessage,
                    user_id,
                }))
                .await;
            tokio::time::sleep(Duration::from_secs(60 * 60 * 12)).await;
        }
    });

    state.subscribe_light(&email, &user_id).await;
    let (mut sender, mut receiver) = socket.split();

    let tx_c = state.tx.clone();
    let mut send_task = tokio::spawn(async move {
        let mut accumulator = 0i64;
        let mut counter = 0u64;
        let _ = sender
            .send(Message::Text(
                WsServerMessage::RequestBandwidthReport.to_string(),
            ))
            .await;
        let mut prev = Utc::now();
        // Send to client - keep alive via ping
        loop {
            let _ = sender.send(Message::Ping(vec![1, 2, 3])).await;
            let now = Utc::now();
            let delta = (now - prev).num_seconds();
            accumulator += delta;
            counter += 1;
            if counter >= counter_period {
                let _ = tx_c
                    .send_async(DBMessage::AggregateAddToMessage(AggregateAddToMessage {
                        msg_type: DBMessageTypes::AggregateAddToMessage,
                        user_id,
                        value: serde_json::Value::from(accumulator),
                        name: AggregateName::Uptime.to_string(),
                    }))
                    .await;
                accumulator = 0;
                counter = 0;
            }
            prev = Utc::now();
            let _ = sender.send(Message::Text("ping".to_string())).await;
            tokio::time::sleep(Duration::from_millis(sleep)).await;
        }
    });

    let tx_c = state.tx.clone();
    let workers = state.workers.clone();
    let mut recv_task = tokio::spawn(async move {
        let workers = workers.clone();
        // Receive from client
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(txt) => {
                    if let Ok(msg) = serde_json::from_str::<WsClientMessage>(&txt) {
                        match msg {
                            WsClientMessage::ReportTwitterCreds(_report) => {
                                let mut workers = workers.write().await;
                                workers.insert(user_id, None);
                            }
                            WsClientMessage::ReportBandwidth(report) => {
                                let mut messages: Vec<DBMessage> = Vec::with_capacity(10);
                                messages.push(DBMessage::AggregateSetToMessage(
                                    AggregateSetToMessage {
                                        msg_type: DBMessageTypes::AggregateSetToMessage,
                                        user_id,
                                        value: serde_json::Value::from(report.download_speed),
                                        name: AggregateName::Download.to_string(),
                                    },
                                ));
                                messages.push(DBMessage::AggregateSetToMessage(
                                    AggregateSetToMessage {
                                        msg_type: DBMessageTypes::AggregateSetToMessage,
                                        user_id,
                                        value: serde_json::Value::from(report.upload_speed),
                                        name: AggregateName::Upload.to_string(),
                                    },
                                ));
                                messages.push(DBMessage::AggregateSetToMessage(
                                    AggregateSetToMessage {
                                        msg_type: DBMessageTypes::AggregateSetToMessage,
                                        user_id,
                                        value: serde_json::Value::from(report.latency),
                                        name: AggregateName::Latency.to_string(),
                                    },
                                ));
                                for message in messages {
                                    let _ = tx_c.send_async(message).await;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                Message::Close(_c) => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {
            create_daily_state_task.abort();
            recv_task.abort();
            send_task.abort();
        },
        _ = &mut recv_task => {
            create_daily_state_task.abort();
            send_task.abort();
            recv_task.abort();
        },
        _ = &mut create_daily_state_task => {
            recv_task.abort();
            send_task.abort();
            create_daily_state_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    state.unsubscribe_light(&email, &user_id).await;
    tracing::trace!("Websocket context {ip} destroyed");
}
