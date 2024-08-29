use crate::ws::task_scheduler::TaskScheduler;
use axum::extract::ws::{Message, WebSocket};
use axum_login::tower_sessions::Session;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use dashmap::{DashMap, DashSet};
use futures::stream::SplitSink;
use futures::{Sink, SinkExt};
use rayon::broadcast;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
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
    users: Arc<DashSet<Uuid>>,
}

impl Broadcaster {
    fn new() -> Self {
        let (transmitter, _) = broadcast::channel(10000);
        let tx = transmitter.clone();
        tokio::spawn(async move {
            loop {
                tx.send(String::from("Task")).unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Self {
            transmitter,
            users: Arc::new(DashSet::new()),
        }
    }

    pub fn broadcast(&self, message: String) -> Result<usize, SendError<String>> {
        let subscribers = self.transmitter.send(message.clone())?;
        tracing::info!("Send {message} to {subscribers} subscribers");
        Ok(subscribers)
    }
    pub fn subscribe(&self, user_id: Uuid) -> broadcast::Receiver<String> {
        let is_new = self.users.insert(user_id);
        debug_assert!(is_new);
        self.transmitter.subscribe()
    }

    pub fn unsubscribe(&self, user_id: &Uuid) {
        self.users.remove(user_id);
    }
}
