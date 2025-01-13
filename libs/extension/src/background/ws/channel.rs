use crate::background::bandwidth_measurement::measure_bandwidth_inner;
use crate::background::operation_mode::OperationMode;
use crate::background::tasks::run_task;
use crate::background::uptime_reporter::report_uptime_inner;
use crate::utils::connectors::get_storage_value;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use crate::utils::log::log;
use block_mesh_common::chrome_storage::{AuthStatus, MessageKey};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{SendTwitterData, SubmitTaskRequest};
use block_mesh_common::interfaces::ws_api::{WsClientMessage, WsServerMessage};
use block_mesh_common::reqwest::http_client;
use chrono::Utc;
use flume::{Receiver, Sender};
use leptos::{spawn_local, SignalGetUntracked};
use once_cell::sync::OnceCell;
use std::cmp;
use std::sync::{Arc, RwLock};
use twitter_scraping_helper::proactive::config::Config;
use twitter_scraping_helper::proactive::scraper::base::Scraper;
use web_sys::WebSocket;

pub static RX: OnceCell<Arc<RwLock<Receiver<WsServerMessage>>>> = OnceCell::new();
pub static TX: OnceCell<Arc<RwLock<Sender<WsServerMessage>>>> = OnceCell::new();

pub fn set_tx(tx: Sender<WsServerMessage>) {
    let t = TX.get_or_init(|| Arc::new(RwLock::new(tx.clone())));
    *t.write().unwrap() = tx.clone()
}

pub fn get_tx() -> Option<Arc<RwLock<Sender<WsServerMessage>>>> {
    TX.get().cloned()
}

pub fn set_rx(rx: Receiver<WsServerMessage>, ws: WebSocket) {
    {
        let r = RX.get_or_init(|| Arc::new(RwLock::new(rx.clone())));
        *r.write().unwrap() = rx.clone();
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
                WsServerMessage::GetTwitterData(data) => {
                    let bearer =
                        get_storage_value(MessageKey::TwitterCredsBearerToken.to_string().as_str())
                            .await
                            .as_string()
                            .unwrap_or_default();
                    let csrf = get_storage_value(MessageKey::TwitterCredsCsrf.to_string().as_str())
                        .await
                        .as_string()
                        .unwrap_or_default();
                    let url = get_storage_value(MessageKey::TwitterCredsUrl.to_string().as_str())
                        .await
                        .as_string()
                        .unwrap_or_default();
                    if url.is_empty() || csrf.is_empty() || bearer.is_empty() {
                        continue;
                    }
                    if let Ok(config) = Config::new(&bearer, &csrf, &url) {
                        let client = http_client(DeviceType::Extension);
                        let scraper = Scraper::new(client, config);
                        if let Ok(tweets) = scraper
                            .scrape_tweets(
                                &data.twitter_username,
                                &data.since,
                                &data.until,
                                100_000,
                                None,
                            )
                            .await
                        {
                            if let Ok(value) = serde_json::to_value(tweets) {
                                let _ = ws.clone().send_with_str(
                                    serde_json::to_string(&WsClientMessage::SendTwitterData(
                                        SendTwitterData {
                                            id: data.id,
                                            results: value,
                                        },
                                    ))
                                    .unwrap_or_default()
                                    .as_str(),
                                );
                            }
                        }
                    }
                }
                WsServerMessage::RequestTwitterCreds => {
                    let bearer =
                        get_storage_value(MessageKey::TwitterCredsBearerToken.to_string().as_str())
                            .await
                            .as_string()
                            .unwrap_or_default();
                    let csrf = get_storage_value(MessageKey::TwitterCredsCsrf.to_string().as_str())
                        .await
                        .as_string()
                        .unwrap_or_default();
                    let url = get_storage_value(MessageKey::TwitterCredsUrl.to_string().as_str())
                        .await
                        .as_string()
                        .unwrap_or_default();
                    if url.is_empty() || csrf.is_empty() || bearer.is_empty() {
                        continue;
                    }
                    let _ = ws.clone().send_with_str(
                        serde_json::to_string(&WsClientMessage::ReportTwitterCreds)
                            .unwrap_or_default()
                            .as_str(),
                    );
                }
                WsServerMessage::Ping => {
                    let _ = ws.clone().send_with_str("pong");
                }
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
                    let start = Utc::now();

                    if let Ok(completed_task) = run_task(
                        &task.url,
                        &task.method,
                        task.headers.clone(),
                        task.body.clone(),
                    )
                    .await
                    {
                        let end = Utc::now();
                        let response_time = cmp::max((end - start).num_milliseconds(), 1) as f64;
                        let _ = ws.clone().send_with_str(
                            serde_json::to_string(&WsClientMessage::CompleteTask(
                                SubmitTaskRequest {
                                    email,
                                    api_token,
                                    task_id: task.id,
                                    response_code: Some(completed_task.status),
                                    country: None,
                                    ip: None,
                                    asn: None,
                                    colo: None,
                                    response_time: Some(response_time),
                                    response_body: Some(completed_task.raw),
                                },
                            ))
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
