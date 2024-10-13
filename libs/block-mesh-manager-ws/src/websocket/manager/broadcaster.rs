use block_mesh_common::interfaces::ws_api::WsServerMessage;
use block_mesh_manager_database_domain::domain::task_limit::TaskLimit;
use dashmap::DashMap;
use futures::future::join_all;
use sqlx::types::chrono::NaiveDate;
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::{broadcast, mpsc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Broadcaster<T: Hash + Eq + Clone> {
    pub global_transmitter: broadcast::Sender<WsServerMessage>,
    pub sockets: Arc<DashMap<T, mpsc::Sender<WsServerMessage>>>,
    pub queue: Arc<Mutex<VecDeque<T>>>,
    pub users_limit: Arc<DashMap<(Uuid, NaiveDate), TaskLimit>>,
}

impl<T: Hash + Eq + Clone> Default for Broadcaster<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq + Clone> Broadcaster<T> {
    pub fn new() -> Self {
        let (global_transmitter, _) = broadcast::channel(10000);
        Self {
            global_transmitter,
            sockets: Arc::new(DashMap::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            users_limit: Arc::new(DashMap::new()),
        }
    }
    pub fn broadcast(&self, message: WsServerMessage) -> Result<usize, SendError<WsServerMessage>> {
        let subscribers = self.global_transmitter.send(message.clone())?;
        tracing::info!("Sent {message:?} to {subscribers} subscribers");
        Ok(subscribers)
    }

    pub async fn batch(&self, message: WsServerMessage, targets: &[T]) {
        join_all(targets.iter().filter_map(|target| {
            if let Some(entry) = self.sockets.get(target) {
                let sink_tx = entry.value().clone();
                let msg = message.clone();
                let future = async move {
                    if let Err(_error) = sink_tx.send(msg).await {
                        tracing::error!("Batch broadcast failed");
                    }
                };
                Some(future)
            } else {
                None
            }
        }))
        .await;
    }

    pub async fn move_queue(&self, count: usize) -> Vec<T> {
        let queue = &mut self.queue.lock().await;
        let count = count.min(queue.len());
        let drained: Vec<T> = queue.drain(0..count).collect();
        queue.extend(drained.clone());
        drained
    }

    pub async fn broadcast_to_user(
        &self,
        messages: impl IntoIterator<Item = WsServerMessage> + Clone,
        id: &T,
    ) {
        let entry = self.sockets.get(id);
        let msgs = messages.clone();
        if let Some(entry) = entry {
            let tx = entry.value().clone();
            for msg in msgs {
                if let Err(error) = tx.send(msg).await {
                    tracing::error!("Error while queuing WS message: {error}");
                }
            }
        }
    }

    /// returns a number of nodes to which [`WsServerMessage`]s were sent
    pub async fn queue_multiple(
        &self,
        messages: impl IntoIterator<Item = WsServerMessage> + Clone,
        count: usize,
    ) -> Vec<T> {
        let drained = self.move_queue(count).await;
        join_all(drained.clone().into_iter().filter_map(|id| {
            if let Some(entry) = self.sockets.get(&id) {
                let tx = entry.value().clone();
                let msgs = messages.clone();
                Some(async move {
                    for msg in msgs {
                        if let Err(error) = tx.send(msg).await {
                            tracing::error!("Error while queuing WS message: {error}");
                        }
                    }
                })
            } else {
                None
            }
        }))
        .await;
        drained
    }

    pub async fn subscribe(
        &self,
        key: T,
        sink_sender: mpsc::Sender<WsServerMessage>,
    ) -> broadcast::Receiver<WsServerMessage> {
        let _ = self.sockets.insert(key.clone(), sink_sender.clone());
        let queue = &mut self.queue.lock().await;
        queue.push_back(key);
        self.global_transmitter.subscribe()
    }

    pub async fn unsubscribe(&self, key: &T) {
        self.sockets.remove(key);
        let queue = &mut self.queue.lock().await;
        if let Some(pos) = queue.iter().position(|x| x == key) {
            queue.remove(pos);
        } else {
            tracing::error!("Failed to remove a socket from the queue");
        }
    }
}
