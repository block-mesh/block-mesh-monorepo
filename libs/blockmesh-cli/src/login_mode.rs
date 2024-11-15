use crate::helpers::{
    get_polling_interval, login_to_network, report_uptime, run_task, submit_bandwidth, task_poller,
};
use anyhow::anyhow;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::feature_flag_client::get_flag_value;
use block_mesh_common::interfaces::server_api::{
    ClientsMetadata, LoginForm, ReportBandwidthRequest, ReportUptimeRequest, SubmitTaskRequest,
};
use block_mesh_common::interfaces::ws_api::{WsClientMessage, WsServerMessage};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use logger_general::tracing::setup_tracing;
use rand::{thread_rng, Rng};
use reqwest_websocket::{Message, RequestBuilderExt};
use speed_test::download::test_download;
use speed_test::latency::test_latency;
use speed_test::metadata::fetch_metadata;
use speed_test::upload::test_upload;
use speed_test::Metadata;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Notify;
use tokio::time::Instant;
use uuid::Uuid;

pub async fn login_mode(
    url: &str,
    email: &str,
    password: &str,
    depin_aggregator: Option<String>,
) -> anyhow::Result<ExitCode> {
    let url = url.to_string();
    let email = email.to_string();
    info!("CLI running with url {}", url);
    let api_token = match login_to_network(
        &url,
        LoginForm {
            email: email.to_string(),
            password: password.to_string(),
        },
    )
    .await
    {
        Ok(api_token) => api_token,
        Err(_) => {
            setup_tracing(Uuid::default(), DeviceType::Cli);
            tracing::error!("Failed to login, did you register on {}/register ?", url);
            return Ok(ExitCode::FAILURE);
        }
    };
    setup_tracing(api_token, DeviceType::Cli);

    info!("Login successful");
    info!("CLI starting");
    let session_metadata = ClientsMetadata {
        depin_aggregator,
        device_type: DeviceType::Cli,
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    };
    let mut prev_is_ws_feature: Option<bool> = None;
    let stop_notifier = Arc::new(Notify::new());
    loop {
        let is_ws_feature = is_ws_feature_connection().await.unwrap_or_default();
        if prev_is_ws_feature.is_none()
            || (prev_is_ws_feature.is_some() && is_ws_feature != prev_is_ws_feature.unwrap())
        {
            prev_is_ws_feature = Some(is_ws_feature);
            stop_notifier.notify_waiters();
            if is_ws_feature {
                tracing::info!("Starting WebSocket");
                connect_ws(
                    url.clone(),
                    email.clone(),
                    api_token,
                    session_metadata.clone(),
                    stop_notifier.clone(),
                )
                .await?;
            } else {
                tracing::info!("Polling");
                poll(
                    url.clone(),
                    email.clone(),
                    api_token,
                    session_metadata.clone(),
                    stop_notifier.clone(),
                )
                .await;
            }
        }
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
async fn connect_ws(
    url: String,
    email: String,
    api_token: Uuid,
    _session_metadata: ClientsMetadata,
    stop_notifier: Arc<Notify>,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let url = url
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    let url = format!("{url}/ws?email={email}&api_token={api_token}");
    let ws = client
        .get(&url)
        .upgrade()
        .send()
        .await?
        .into_websocket()
        .await?;
    let (mut sink, mut stream) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<WsClientMessage>(10);
    let messenger_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(payload) = serde_json::to_string(&msg) {
                let _ = sink.send(Message::Text(payload)).await;
            }
        }
    });
    let worker_handle = tokio::spawn(async move {
        while let Some(msg) = stream.try_next().await.unwrap_or_default() {
            match msg {
                Message::Text(text) => {
                    if let Ok(payload) = serde_json::from_str::<WsServerMessage>(text.as_str()) {
                        if matches!(payload, WsServerMessage::CloseConnection) {
                            break;
                        }
                        tracing::info!("Got WS message {:#?}", payload);
                        handle_ws_message(payload, tx.clone(), email.clone(), api_token).await;
                    }
                }
                Message::Binary(_) => {}
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Close { .. } => break,
            }
        }
    });
    tokio::select! {
        o = messenger_handle => warn!("Messenger handle stopped receiving messages {o:?}"),
        o = worker_handle => warn!("WS Connection was closed {o:?}"),
        o = stop_notifier.notified() => warn!("WS connection was interrupted by a feature flag change {o:?}")
    }
    Ok(())
}

async fn handle_ws_message(
    payload: WsServerMessage,
    tx: Sender<WsClientMessage>,
    email: String,
    api_token: Uuid,
) {
    let _task = tokio::spawn(async move {
        match payload {
            WsServerMessage::AssignTask(task) => {
                let Metadata {
                    country,
                    ip,
                    asn,
                    colo,
                    ..
                } = fetch_metadata().await.unwrap_or_default();
                let task_start = Instant::now();
                let completed_task = run_task(&task.url, &task.method, task.headers, task.body)
                    .await
                    .unwrap();
                let response_time = Some(std::cmp::max(task_start.elapsed().as_millis(), 1) as f64);
                let report = SubmitTaskRequest {
                    email: email.clone(),
                    api_token,
                    task_id: task.id,
                    response_code: Some(completed_task.status),
                    country: Some(country),
                    ip: Some(ip),
                    asn: Some(asn),
                    colo: Some(colo),
                    response_time,
                    response_body: Some(completed_task.raw),
                };
                let _ = tx.send(WsClientMessage::CompleteTask(report)).await;
            }
            WsServerMessage::RequestBandwidthReport => {
                let download_speed = test_download(100_000).await.unwrap_or_default();
                let upload_speed = test_upload(100_000).await.unwrap_or_default();
                let latency = test_latency().await.unwrap_or_default();
                let metadata = fetch_metadata().await.unwrap_or_default();

                let report = ReportBandwidthRequest {
                    email: email.to_string(),
                    api_token,
                    download_speed,
                    upload_speed,
                    latency,
                    city: metadata.city,
                    country: metadata.country,
                    ip: metadata.ip,
                    asn: metadata.asn,
                    colo: metadata.colo,
                };
                let _ = tx.send(WsClientMessage::ReportBandwidth(report)).await;
            }
            WsServerMessage::RequestUptimeReport => {
                let cf_meta = fetch_metadata().await.unwrap_or_default();
                let report = ReportUptimeRequest {
                    email: email.clone(),
                    api_token,
                    ip: Some(cf_meta.ip).filter(|ip| !ip.is_empty()),
                };
                let _ = tx.send(WsClientMessage::ReportUptime(report)).await;
            }
            WsServerMessage::Ping => {
                let _ = tx.send(WsClientMessage::Ping).await;
            }
            WsServerMessage::CloseConnection => {}
        }
    });
}
async fn is_ws_feature_connection() -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let response = get_flag_value("cli_use_websocket", &client, DeviceType::Cli).await?;
    if let Some(res) = response {
        let is_enabled = res.as_bool().unwrap_or_default();
        if !is_enabled {
            return Ok(false);
        }
    } else {
        return Err(anyhow!(
            "Failed to fetch 'use_websocket' feature flag: {response:#?}"
        ));
    }

    let response = get_flag_value("cli_use_websocket_percent", &client, DeviceType::Cli).await?;
    if let Some(res) = response {
        let percentage: u64 = res.as_u64().unwrap_or_default();
        let mut rng = thread_rng();
        let probe: u64 = rng.gen_range(0..100);
        Ok(probe < percentage)
    } else {
        Err(anyhow!(
            "Failed to fetch 'use_websocket_percent' feature flag: {response:#?}"
        ))
    }
}
async fn poll(
    url: String,
    email: String,
    api_token: Uuid,
    session_metadata: ClientsMetadata,
    stop_notifier: Arc<Notify>,
) {
    let url = Arc::new(url);
    let email = Arc::new(email.to_string());
    let api_token = Arc::new(api_token.to_string());
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let task_poller = tokio::spawn(async move {
        loop {
            let _ = task_poller(&u, e.as_ref(), a.as_ref()).await;
            let polling_interval = get_polling_interval().await;
            tokio::time::sleep(Duration::from_secs(polling_interval as u64)).await;
        }
    });
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let uptime_poller = tokio::spawn(async move {
        loop {
            let _ = report_uptime(&u, e.as_ref(), a.as_ref(), session_metadata.clone()).await;
            let polling_interval = get_polling_interval().await;
            tokio::time::sleep(Duration::from_secs(polling_interval as u64)).await;
        }
    });
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let bandwidth_poller = tokio::spawn(async move {
        loop {
            let _ = submit_bandwidth(&u, e.as_ref(), a.as_ref()).await;
            let polling_interval = get_polling_interval().await;
            tokio::time::sleep(Duration::from_secs(polling_interval as u64)).await;
        }
    });
    tokio::select! {
        o = task_poller => error!("task_poller failed {:?}", o),
        o = uptime_poller => error!("uptime_poller failed {:?}", o),
        o = bandwidth_poller => error!("bandwidth_poller failed {:?}", o),
        o = stop_notifier.notified() => info!("Polling was interrupted by a feature flag change: {:?} ", o)
    }
}
