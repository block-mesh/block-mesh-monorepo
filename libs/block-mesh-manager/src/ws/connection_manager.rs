use crate::ws::task_scheduler::TaskScheduler;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use futures::future::join_all;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    pub broadcaster: Broadcaster,
    pub task_scheduler: TaskScheduler<GetTaskResponse>,
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
    sockets: Arc<DashMap<Uuid, tokio::sync::mpsc::Sender<WsServerMessage>>>,
}

impl Broadcaster {
    fn new() -> Self {
        let (transmitter, _) = broadcast::channel(10000);
        let tx = transmitter.clone();
        // demo
        let _broadcast_handle = tokio::spawn(async move {
            loop {
                tracing::info!("Sending demo broadcast");
                println!("Sending demo broadcast");
                if tx.send(WsServerMessage::RequestBandwidthReport).is_err() {
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Self {
            transmitter,
            sockets: Arc::new(DashMap::new()),
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

    pub fn subscribe(
        &self,
        user_id: Uuid,
        sink_sender: tokio::sync::mpsc::Sender<WsServerMessage>,
    ) -> broadcast::Receiver<WsServerMessage> {
        let old_value = self.sockets.insert(user_id, sink_sender);
        debug_assert!(old_value.is_none());
        self.transmitter.subscribe()
    }

    pub fn unsubscribe(&self, user_id: &Uuid) {
        self.sockets.remove(user_id);
    }
}
