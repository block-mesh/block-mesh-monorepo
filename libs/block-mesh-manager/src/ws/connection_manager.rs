use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::domain::aggregate::AggregateName;
use crate::ws::task_scheduler::TaskScheduler;
use anyhow::Context;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::VecDeque;
use std::env;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::mpsc;
use tokio::sync::watch::Sender;
use tokio::sync::{broadcast, watch};
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

    pub async fn cron_reports(
        &mut self,
        period: Duration,
        messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
        window_size: usize,
        pool: PgPool,
    ) -> anyhow::Result<CronReportsController> {
        self.broadcaster
            .cron_reports(period, messages, window_size, pool)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct CronReportsController {
    stats_receiver: watch::Receiver<CronReportStats>,
}

impl CronReportsController {
    fn new(stats_receiver: watch::Receiver<CronReportStats>) -> Self {
        Self { stats_receiver }
    }

    pub fn stats(&mut self) -> CronReportStats {
        self.stats_receiver.borrow_and_update().clone()
    }
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub cron_reports_controller: Option<CronReportsController>,
}

impl Broadcaster {
    fn new() -> Self {
        let (global_transmitter, _) = broadcast::channel(10000);
        Self {
            global_transmitter,
            sockets: Arc::new(DashMap::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            cron_reports_controller: None,
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

    pub async fn cron_reports(
        &mut self,
        period: Duration,
        messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
        window_size: usize,
        pool: PgPool,
    ) -> anyhow::Result<CronReportsController> {
        let (stats_tx, stats_rx) = tokio::sync::watch::channel(CronReportStats::new(
            messages.clone().into(),
            window_size,
            0,
        ));
        let broadcaster = self.clone();
        let pool = pool.clone();
        let user_id = Uuid::parse_str(
            env::var(BLOCKMESH_SERVER_UUID_ENVAR)
                .context("Could not find SERVER_UUID env var")?
                .as_str(),
        )
        .context("SERVER_UUID evn var contains invalid UUID value")?;

        tokio::spawn(async move {
            let _ = settings_loop(
                &pool,
                &user_id,
                period,
                messages,
                window_size,
                broadcaster,
                stats_tx,
            )
            .await;
        });

        let controller = CronReportsController::new(stats_rx);
        self.cron_reports_controller = Some(controller.clone());
        Ok(controller)
    }
}

async fn settings_loop(
    pool: &PgPool,
    user_id: &Uuid,
    period: Duration,
    messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
    window_size: usize,
    broadcaster: Broadcaster,
    stats_tx: Sender<CronReportStats>,
) -> anyhow::Result<()> {
    let _ = fetch_latest_cron_settings(pool, user_id)
        .await
        .inspect_err(|e| tracing::error!("DB: {e}"));
    let mut transaction = pool.begin().await?;
    update_aggregate(
        &mut transaction,
        user_id,
        &serde_json::to_value(CronReportSettings::new(
            Some(period),
            Some(messages.clone().into()),
            Some(window_size),
        ))
        .context("Failed to parse cron report settings")?,
    )
    .await?;
    let messages = messages.into();
    loop {
        let messages = messages.clone();
        let settings = fetch_latest_cron_settings(pool, user_id).await?;
        let new_period = settings.period.unwrap_or(period);
        let new_messages = settings.messages.unwrap_or(messages);
        let new_window_size = settings.window_size.unwrap_or(window_size);
        let sent_messages_count = broadcaster
            .queue_multiple(new_messages.clone(), window_size)
            .await;
        if let Err(error) = stats_tx.send(CronReportStats::new(
            new_messages,
            new_window_size,
            sent_messages_count,
        )) {
            // TODO (send_if_modified, send_modify, or send_replace) can be used instead
            tracing::error!("Could not sent stats, no watchers: {error}");
        }
        tokio::time::sleep(new_period).await;
    }
}

async fn fetch_latest_cron_settings(
    pool: &PgPool,
    user_id: &Uuid,
) -> anyhow::Result<CronReportSettings> {
    let aggregate =
        get_or_create_aggregate_by_user_and_name_pool(pool, AggregateName::CronReports, user_id)
            .await?;
    let settings: CronReportSettings = serde_json::from_value(aggregate.value)?;
    Ok(settings)
}
