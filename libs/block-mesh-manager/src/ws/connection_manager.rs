use crate::ws::task_scheduler::TaskScheduler;
use axum::extract::ws::{Message, WebSocket};
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::WsMessage;
use dashmap::{DashMap, DashSet};
use futures::future::join_all;
use futures::task::SpawnExt;
use futures::SinkExt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use tokio::task::{JoinHandle, JoinSet};
use tracing::{error, trace, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    pub broadcaster: Broadcaster,
    pub task_scheduler: TaskScheduler<GetTaskResponse>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            broadcaster: Broadcaster::new(),
            task_scheduler: TaskScheduler::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Broadcaster {
    transmitter: broadcast::Sender<String>,
    users: Arc<DashMap<Uuid, tokio::sync::mpsc::Sender<WsMessage>>>, // Arc<RwLock<HasSet<T>>> DashMap<Uuid, tokio::sync::mpsc::Sender<WsMessage>
}

impl Broadcaster {
    fn new() -> Self {
        let (transmitter, _) = broadcast::channel(10000);
        let tx = transmitter.clone();
        let broadcast_handle = tokio::spawn(async move {
            loop {
                tracing::info!("Sending demo broadcast");
                println!("Sending demo broadcast");
                if tx.send(String::from("Task")).is_err() {
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Self {
            transmitter,
            users: Arc::new(DashMap::new()),
        }
    }
    pub fn broadcast(&self, message: String) -> Result<usize, SendError<String>> {
        let subscribers = self.transmitter.send(message.clone())?;
        tracing::info!("Send {message} to {subscribers} subscribers");
        Ok(subscribers)
    }

    pub async fn batch(&self, message: WsMessage, targets: &[Uuid]) {
        join_all(targets.iter().filter_map(|target| {
            if let Some(entry) = self.users.get(target) {
                let sink_tx = entry.value().clone();
                let msg = message.clone();
                let future = async move {
                    sink_tx.send(msg).await.unwrap();
                };
                Some(future)
            } else {
                None
            }
        }))
        .await;
    }

    pub fn subscribe(
        &self,
        user_id: Uuid,
        sink_sender: tokio::sync::mpsc::Sender<WsMessage>,
    ) -> broadcast::Receiver<String> {
        let old_value = self.users.insert(user_id, sink_sender);
        debug_assert!(old_value.is_none());
        self.transmitter.subscribe()
    }

    pub fn unsubscribe(&self, user_id: &Uuid) {
        self.users.remove(user_id);
    }
}
