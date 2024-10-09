use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use axum::extract::ws::Message;
use block_mesh_common::interfaces::server_api::HandlerMode;
use block_mesh_common::interfaces::ws_api::WsClientMessage;
use block_mesh_manager_database_domain::domain::report_uptime_content::report_uptime_content;
use block_mesh_manager_database_domain::domain::submit_bandwidth_content::submit_bandwidth_content;
use block_mesh_manager_database_domain::domain::submit_task_content::submit_task_content;
use std::ops::ControlFlow;
use std::sync::Arc;

/// helper to print contents of messages to stdout. Has special treatment for Close.
#[tracing::instrument(name = "process_message", skip_all)]
pub async fn process_message(
    msg: Message,
    ip: String,
    state: Arc<AppState>,
) -> ControlFlow<(), Option<WsClientMessage>> {
    match msg {
        Message::Text(text) => {
            let ws_client_message = process_client_message(&text, ip, state).await;
            return ControlFlow::Continue(ws_client_message);
        }
        Message::Binary(bytes) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", ip, bytes.len(), bytes);
        }
        Message::Close(frame) => {
            if let Some(cf) = frame {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    ip,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {ip} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(bytes) => {
            tracing::info!(">>> {ip} sent pong with {bytes:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(bytes) => {
            tracing::info!(">>> {ip} sent ping with {bytes:?}");
        }
    }
    ControlFlow::Continue(None)
}

#[tracing::instrument(name = "process_client_message", skip_all)]
async fn process_client_message(
    text: &str,
    ip: String,
    state: Arc<AppState>,
) -> Option<WsClientMessage> {
    if text == "pong" {
        return Some(WsClientMessage::Ping);
    }
    match serde_json::from_str::<WsClientMessage>(text) {
        Ok(message) => {
            match &message {
                WsClientMessage::CompleteTask(query) => {
                    let _ = submit_task_content(
                        &state.pool,
                        query.clone(),
                        None,
                        HandlerMode::WebSocket,
                    )
                    .await;
                }
                WsClientMessage::ReportBandwidth(body) => {
                    let _ = submit_bandwidth_content(&state.pool, body.clone()).await;
                }
                WsClientMessage::ReportUptime(query) => {
                    let _ = report_uptime_content(
                        &state.pool,
                        ip.clone(),
                        query.clone(),
                        None,
                        HandlerMode::WebSocket,
                        get_envar("POLLING_INTERVAL")
                            .await
                            .parse()
                            .unwrap_or(120_000.0),
                        get_envar("INTERVAL_FACTOR").await.parse().unwrap_or(10.0),
                    )
                    .await;
                }
                WsClientMessage::Ping => {
                    return Some(WsClientMessage::Ping);
                }
            }
            return Some(message);
        }
        Err(_) => {
            if text != "ping" {
                tracing::info!("Invalid Message => {}", text)
            }
        }
    }
    None
}
