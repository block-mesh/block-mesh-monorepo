use crate::helpers::{
    get_polling_interval, login_to_network, report_uptime, run_task, submit_bandwidth, task_poller,
};
use anyhow::anyhow;
use block_mesh_common::constants::DeviceType;
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
    };
    if is_ws_feature_connection().await? {
        connect_ws(url, email, api_token, session_metadata).await?;
    } else {
        poll(url, email, api_token, session_metadata).await;
    }
    Ok(ExitCode::SUCCESS)
}
async fn connect_ws(
    url: String,
    email: String,
    api_token: Uuid,
    _session_metadata: ClientsMetadata,
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
    while let Some(msg) = stream.try_next().await? {
        match msg {
            Message::Text(text) => {
                if let Ok(payload) = serde_json::from_str::<WsServerMessage>(text.as_str()) {
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
                            let completed_task =
                                run_task(&task.url, &task.method, task.headers, task.body).await?;
                            let response_time =
                                Some(std::cmp::max(task_start.elapsed().as_millis(), 1) as f64);
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
                            sink.send(Message::Text(serde_json::to_string(
                                &WsClientMessage::CompleteTask(report),
                            )?))
                            .await?;
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
                            sink.send(Message::Text(serde_json::to_string(
                                &WsClientMessage::ReportBandwidth(report),
                            )?))
                            .await?;
                        }
                        WsServerMessage::RequestUptimeReport => {
                            let cf_meta = fetch_metadata().await.unwrap_or_default();
                            let report = ReportUptimeRequest {
                                email: email.clone(),
                                api_token,
                                ip: Some(cf_meta.ip).filter(|ip| !ip.is_empty()),
                            };
                            sink.send(Message::Text(serde_json::to_string(
                                &WsClientMessage::ReportUptime(report),
                            )?))
                            .await?;
                        }
                        WsServerMessage::CloseConnection => break,
                    }
                }
            }
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close { .. } => break,
        }
    }
    // TODO: close WS
    todo!()
}
async fn is_ws_feature_connection() -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://feature-flags.blockmesh.xyz/read-flag/use_websocket")
        .send()
        .await?;
    if response.status().is_success() {
        let value = response.text().await?;
        let is_enabled: bool = value.trim().parse()?;
        if !is_enabled {
            return Ok(false);
        }
    } else {
        return Err(anyhow!(
            "Failed to fetch 'use_websocket' feature flag: {response:#?}"
        ));
    }

    let response = client
        .get("https://feature-flags.blockmesh.xyz/read-flag/use_websocket_percent")
        .send()
        .await?;
    if response.status().is_success() {
        let value = response.text().await?;
        let percentage: u32 = value.parse()?;
        let probe = thread_rng().gen_range(0, 100);
        Ok(probe < percentage)
    } else {
        Err(anyhow!(
            "Failed to fetch 'use_websocket_percent' feature flag: {response:#?}"
        ))
    }
}
async fn poll(url: String, email: String, api_token: Uuid, session_metadata: ClientsMetadata) {
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
        o = bandwidth_poller => error!("bandwidth_poller failed {:?}", o)
    }
}
