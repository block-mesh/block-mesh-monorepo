use crate::ws::task_scheduler::TaskScheduler;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use futures::future::join_all;
use std::cmp::min;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    pub broadcaster: Broadcaster,
    pub task_scheduler: TaskScheduler<WsServerMessage>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
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
    transmitter: broadcast::Sender<WsServerMessage>,
    sockets: Arc<DashMap<Uuid, mpsc::Sender<WsServerMessage>>>,
    queue: Arc<Mutex<VecDeque<Uuid>>>,
}

impl Broadcaster {
    fn new() -> Self {
        let (transmitter, _) = broadcast::channel(10000);
        Self {
            transmitter,
            sockets: Arc::new(DashMap::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn broadcast(&self, message: WsServerMessage) -> Result<usize, SendError<WsServerMessage>> {
        let subscribers = self.transmitter.send(message.clone())?;
        tracing::info!("Sent {message:?} to {subscribers} subscribers");
        Ok(subscribers)
    }

    pub async fn batch(&self, message: WsServerMessage, targets: &[Uuid]) {
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
    pub fn move_queue(&self, count: usize) -> Vec<Uuid> {
        let queue = &mut self.queue.lock().unwrap();
        let count = count.min(queue.len());
        let drained: Vec<Uuid> = queue.drain(0..count).collect();
        queue.extend(drained.iter());
        drained
    }
    pub async fn queue(&self, message: WsServerMessage, count: usize) {
        let drained = self.move_queue(count);
        join_all(drained.into_iter().map(|user_id| {
            let entry = self.sockets.get(&user_id).unwrap();
            let tx = entry.value().clone();
            let msg = message.clone();
            async move { tx.send(msg).await }
        }))
        .await;
    }
    pub async fn queue_multiple(&self, messages: &[WsServerMessage], count: usize) {
        let drained = self.move_queue(count);
        join_all(drained.into_iter().map(|user_id| {
            let entry = self.sockets.get(&user_id).unwrap();
            let tx = entry.value().clone();
            async move {
                for msg in messages {
                    tx.send(msg.clone()).await.unwrap(); // FIXME concurrency
                }
            }
        }))
        .await;
    }

    pub fn subscribe(
        &self,
        user_id: Uuid,
        sink_sender: tokio::sync::mpsc::Sender<WsServerMessage>,
    ) -> broadcast::Receiver<WsServerMessage> {
        let old_value = self.sockets.insert(user_id, sink_sender.clone());
        let queue = &mut self.queue.lock().unwrap();
        queue.push_back(user_id);
        debug_assert!(old_value.is_none());
        self.transmitter.subscribe()
    }

    pub fn unsubscribe(&self, user_id: &Uuid) {
        self.sockets.remove(user_id);
        let queue = &mut self.queue.lock().unwrap();
        let pos = queue.iter().position(|x| x == user_id).unwrap();
        queue.remove(pos);
    }
}
