use crate::background::bandwidth_measurement::measure_bandwidth_inner;
use crate::background::operation_mode::OperationMode;
use crate::background::tasks::run_task;
use crate::background::uptime_reporter::report_uptime_inner;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use crate::utils::log::log;
use block_mesh_common::chrome_storage::AuthStatus;
use block_mesh_common::interfaces::ws_api::{WsClientMessage, WsServerMessage};
use flume::{Receiver, Sender};
use leptos::{spawn_local, SignalGetUntracked};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use web_sys::WebSocket;

pub static RX: OnceCell<Arc<Mutex<Receiver<WsServerMessage>>>> = OnceCell::new();
pub static TX: OnceCell<Arc<Mutex<Sender<WsServerMessage>>>> = OnceCell::new();

pub fn set_tx(tx: Sender<WsServerMessage>) {
    let t = TX.get_or_init(|| Arc::new(Mutex::new(tx.clone())));
    *t.lock().unwrap() = tx.clone()
}

pub fn get_tx() -> Option<Arc<Mutex<Sender<WsServerMessage>>>> {
    TX.get().cloned()
}

pub fn set_rx(rx: Receiver<WsServerMessage>, ws: WebSocket) {
    {
        let r = RX.get_or_init(|| Arc::new(Mutex::new(rx.clone())));
        *r.lock().unwrap() = rx.clone();
    }

    spawn_local(async move {
        let rx = rx.clone();
        while let Ok(msg) = rx.recv_async().await {
            if matches!(msg, WsServerMessage::CloseConnection) {
                if let Err(error) = ws.close() {
                    log!("Error while closing WS: {error:?}");
                }
                return;
            }
            let app_state = ExtensionWrapperState::default();
            app_state.init_with_storage().await;
            log!("RX msg {:?} - {:?}", msg, app_state);

            if !app_state.has_api_token() {
                continue;
            }
            if app_state.status.get_untracked() == AuthStatus::LoggedOut {
                continue;
            }
            let base_url = app_state.blockmesh_url.get_untracked();
            let email = app_state.email.get_untracked();
            let api_token = app_state.api_token.get_untracked();

            match msg {
                WsServerMessage::RequestUptimeReport => {
                    if let Some(r) =
                        report_uptime_inner(&base_url, &email, &api_token, OperationMode::WebSocket)
                            .await
                    {
                        let _ = ws.clone().send_with_str(
                            serde_json::to_string(&WsClientMessage::ReportUptime(r))
                                .unwrap_or_default()
                                .as_str(),
                        );
                    }
                }
                WsServerMessage::RequestBandwidthReport => {
                    if let Some(r) = measure_bandwidth_inner(
                        &base_url,
                        &email,
                        &api_token,
                        OperationMode::WebSocket,
                    )
                    .await
                    {
                        let _ = ws.clone().send_with_str(
                            serde_json::to_string(&WsClientMessage::ReportBandwidth(r))
                                .unwrap_or_default()
                                .as_str(),
                        );
                    }
                }
                WsServerMessage::AssignTask(task) => {
                    if let Ok(completed_task) = run_task(
                        &task.url,
                        &task.method,
                        task.headers.clone(),
                        task.body.clone(),
                    )
                    .await
                    {
                        let _ = ws.clone().send_with_str(
                            serde_json::to_string(&WsClientMessage::CompleteTask(completed_task))
                                .unwrap_or_default()
                                .as_str(),
                        );
                    }
                }
                WsServerMessage::CloseConnection => {
                    if let Err(error) = ws.close() {
                        log!("Error while closing WS: {error:?}");
                    }
                    return;
                }
            }
        }
    });
}

pub fn setup_channels(ws: WebSocket) {
    let (tx, rx) = flume::unbounded::<WsServerMessage>();
    set_tx(tx);
    set_rx(rx, ws.clone());
}
