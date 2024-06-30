#![allow(clippy::let_underscore_future)]

use crate::blockmesh::blockmesh_task_monitor::{report_uptime, task_poller};
use crate::ore::ore_process_wrapper::{kill_ore_process, start_ore_process};
use crate::tauri_state::{AppState, ChannelMessage};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub async fn channel_receiver(mut rx: Receiver<ChannelMessage>, app_state: Arc<Mutex<AppState>>) {
    while let Ok(msg) = rx.recv().await {
        tracing::info!("channel_receiver starting to process msg => {}", msg);
        match msg {
            ChannelMessage::StartOre => {
                let _ = kill_ore_process(app_state.clone()).await;
                let _ = start_ore_process(app_state.clone()).await;
            }
            ChannelMessage::ShutdownOre => {
                let _ = kill_ore_process(app_state.clone()).await;
            }
            ChannelMessage::StartUptime => {
                let state = app_state.clone();
                let handle = tauri::async_runtime::spawn(async move {
                    let mut s = state.lock().await;
                    if let Some(h) = &s.uptime_handle {
                        h.abort();
                        s.uptime_handle = None;
                    }
                    let email = Arc::new(s.config.email.clone().unwrap_or_default());
                    let api_token = Arc::new(s.config.api_token.clone().unwrap_or_default());
                    drop(s);
                    loop {
                        let _ = report_uptime(email.to_string(), api_token.to_string()).await;
                        sleep(Duration::from_secs(30)).await;
                    }
                });
                let mut s = app_state.lock().await;
                s.uptime_handle = Some(handle);
            }
            ChannelMessage::ShutdownUptime => {
                let state = app_state.clone();
                let mut s = state.lock().await;
                if let Some(h) = &s.uptime_handle {
                    h.abort();
                    s.uptime_handle = None;
                }
            }
            ChannelMessage::StartNode => {}
            ChannelMessage::ShutdownNode => {}
            ChannelMessage::StartTaskPull => {
                let state = app_state.clone();
                let handle = tauri::async_runtime::spawn(async move {
                    let mut s = state.lock().await;
                    if let Some(h) = &s.uptime_handle {
                        h.abort();
                        s.task_puller = None;
                    }
                    let email = Arc::new(s.config.email.clone().unwrap_or_default());
                    let api_token = Arc::new(s.config.api_token.clone().unwrap_or_default());
                    drop(s);
                    loop {
                        let _ = task_poller(email.to_string(), api_token.to_string()).await;
                        sleep(Duration::from_secs(30)).await;
                    }
                });
                let mut s = app_state.lock().await;
                s.task_puller = Some(handle);
            }
            ChannelMessage::ShutdownTaskPull => {
                let state = app_state.clone();
                let mut s = state.lock().await;
                if let Some(h) = &s.uptime_handle {
                    h.abort();
                    s.task_puller = None;
                }
            }
            ChannelMessage::RestartTask => {}
        }
        tracing::info!("channel_receiver finished to process msg => {}", msg);
    }
}
