use crate::ws::task_scheduler::TaskScheduler;
use aws_sdk_sesv2::config::IntoShared;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use futures::future::join_all;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::mpsc;
use tokio::sync::{broadcast, watch};
use tokio::task::JoinHandle;
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

    pub fn cron_reports(
        &self,
        period: Duration,
        messages: impl Into<Vec<WsServerMessage>> + Clone,
        window_size: usize,
    ) -> CronReportsController {
        let broadcaster = self.broadcaster.clone();
        let (settings_tx, mut settings_rx) = mpsc::channel::<CronReportSettings>(10);
        let (stats_tx, stats_rx) = tokio::sync::watch::channel(CronReportStats::new(
            messages.clone().into(),
            window_size,
            0,
        ));
        let period = Arc::new(Mutex::new(period));
        let messages = Arc::new(Mutex::new(messages.into()));
        let window_size = Arc::new(AtomicUsize::new(window_size));
        let channel_task = {
            let period = period.clone();
            let window_size = window_size.clone();
            let messages = messages.clone();
            tokio::spawn(async move {
                while let Some(setting) = settings_rx.recv().await {
                    if let Some(msgs) = setting.messages {
                        *messages.lock().unwrap() = msgs;
                    }
                    if let Some(window_s) = setting.window_size {
                        window_size.store(window_s, Ordering::Relaxed);
                    }
                    if let Some(per) = setting.period {
                        if !per.is_zero() {
                            *period.lock().unwrap() = per;
                        }
                    }
                }
            })
        };
        let cron_task = {
            let period = period.clone();
            let window_size = window_size.clone();
            let messages = messages.clone();
            tokio::spawn(async move {
                loop {
                    let messages = { messages.lock().unwrap().clone() };
                    let window_size = window_size.load(Ordering::Relaxed);
                    let period = { period.lock().unwrap().clone() };
                    let sent_messages_count = broadcaster
                        .queue_multiple(messages.clone(), window_size)
                        .await;
                    if let Err(error) = stats_tx.send(CronReportStats::new(
                        messages,
                        window_size,
                        sent_messages_count,
                    )) {
                        // TODO (send_if_modified, send_modify, or send_replace) can be used instead
                        tracing::error!("Could not sent stats, no watchers: {error}");
                    }
                    tokio::time::sleep(period).await;
                }
                ()
            })
        };

        CronReportsController::new(cron_task, channel_task, settings_tx, stats_rx)
    }
}
pub struct CronReportsController {
    cron_task: JoinHandle<()>,
    channel_task: JoinHandle<()>,
    settings_transmitter: mpsc::Sender<CronReportSettings>,
    stats_receiver: watch::Receiver<CronReportStats>,
}

impl CronReportsController {
    fn new(
        cron_task: JoinHandle<()>,
        channel_task: JoinHandle<()>,
        settings_transmitter: mpsc::Sender<CronReportSettings>,
        stats_receiver: watch::Receiver<CronReportStats>,
    ) -> Self {
        Self {
            cron_task,
            channel_task,
            settings_transmitter,
            stats_receiver,
        }
    }
    pub async fn update(&self, settings: CronReportSettings) {
        if let Err(error) = self.settings_transmitter.send(settings).await {
            tracing::error!("Could not update cron report settings: {error}");
        }
    }

    pub fn stats(&mut self) -> CronReportStats {
        self.stats_receiver.borrow_and_update().clone()
    }
}

#[derive(Debug, Clone)]
pub struct CronReportStats {
    messages: Vec<WsServerMessage>,
    window_size: usize,
    used_window_size: usize,
}

impl CronReportStats {
    fn new(messages: Vec<WsServerMessage>, window_size: usize, used_window_size: usize) -> Self {
        Self {
            messages,
            window_size,
            used_window_size,
        }
    }
}
impl Default for CronReportStats {
    fn default() -> Self {
        Self::new(vec![], 0, 0)
    }
}

pub struct CronReportSettings {
    period: Option<Duration>,
    messages: Option<Vec<WsServerMessage>>,
    window_size: Option<usize>,
}

impl CronReportSettings {
    pub fn new(
        period: Option<Duration>,
        messages: Option<impl Into<Vec<WsServerMessage>>>,
        window_size: Option<usize>,
    ) -> Self {
        Self {
            period,
            messages: messages.map(|m| m.into()),
            window_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Broadcaster {
    global_transmitter: broadcast::Sender<WsServerMessage>,
    sockets: Arc<DashMap<(Uuid, SocketAddr), mpsc::Sender<WsServerMessage>>>,
    queue: Arc<Mutex<VecDeque<(Uuid, SocketAddr)>>>,
}

impl Broadcaster {
    fn new() -> Self {
        let (global_transmitter, _) = broadcast::channel(10000);
        Self {
            global_transmitter,
            sockets: Arc::new(DashMap::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn broadcast(&self, message: WsServerMessage) -> Result<usize, SendError<WsServerMessage>> {
        let subscribers = self.global_transmitter.send(message.clone())?;
        tracing::info!("Sent {message:?} to {subscribers} subscribers");
        Ok(subscribers)
    }

    pub async fn batch(&self, message: WsServerMessage, targets: &[(Uuid, SocketAddr)]) {
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

    pub fn move_queue(&self, count: usize) -> Vec<(Uuid, SocketAddr)> {
        let queue = &mut self.queue.lock().unwrap();
        let count = count.min(queue.len());
        let drained: Vec<(Uuid, SocketAddr)> = queue.drain(0..count).collect();
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

    /// returns a number of nodes to which [`WsServerMessage`]s were sent
    pub async fn queue_multiple(
        &self,
        messages: impl IntoIterator<Item = WsServerMessage> + Clone,
        count: usize,
    ) -> usize {
        let drained = self.move_queue(count);
        let queued_messages_count = drained.len();
        join_all(drained.into_iter().map(|user_id| {
            let entry = self.sockets.get(&user_id).unwrap();
            let tx = entry.value().clone();

            let msgs = messages.clone();
            async move {
                for msg in msgs {
                    if let Err(error) = tx.send(msg).await {
                        tracing::error!("Error while queuing WS message: {error}");
                    }
                }
            }
        }))
        .await;
        queued_messages_count
    }

    pub fn subscribe(
        &self,
        user_id: Uuid,
        socket_addr: SocketAddr,
        sink_sender: mpsc::Sender<WsServerMessage>,
    ) -> broadcast::Receiver<WsServerMessage> {
        let old_value = self
            .sockets
            .insert((user_id, socket_addr), sink_sender.clone());
        let queue = &mut self.queue.lock().unwrap();
        queue.push_back((user_id, socket_addr));
        debug_assert!(old_value.is_none());
        self.global_transmitter.subscribe()
    }

    pub fn unsubscribe(&self, user_id: Uuid, socket_addr: SocketAddr) {
        self.sockets.remove(&(user_id, socket_addr));
        let queue = &mut self.queue.lock().unwrap();
        if let Some(pos) = queue
            .iter()
            .position(|(a, b)| a == &user_id && b == &socket_addr)
        {
            queue.remove(pos);
        } else {
            tracing::error!("Failed to remove a socket from the queue");
        }
    }
}

#[tokio::test]
async fn test_cron_reports() {
    let conn_manager = ConnectionManager::new();
    let (tx, mut rx) = mpsc::channel(10);
    let user_id = Uuid::new_v4();
    let addr = SocketAddr::from_str("127.0.0.1:8000").unwrap();
    conn_manager.broadcaster.subscribe(user_id, addr, tx);
    let mut controller = conn_manager.cron_reports(
        Duration::from_secs(1),
        vec![
            WsServerMessage::RequestUptimeReport,
            WsServerMessage::RequestBandwidthReport,
        ],
        10,
    );
    let msg = rx.recv().await.unwrap();
    conn_manager.broadcaster.unsubscribe(user_id, addr);
    let stats = controller.stats();
    println!("{stats:?}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    controller
        .update(CronReportSettings {
            period: None,
            messages: None,
            window_size: Some(10),
        })
        .await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let stats = controller.stats();
    println!("{stats:?}");
}
