use block_mesh_common::interfaces::ws_api::WsServerMessage;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct CronReportsController {
    stats_receiver: watch::Receiver<CronReportStats>,
}

impl CronReportsController {
    pub fn new(stats_receiver: watch::Receiver<CronReportStats>) -> Self {
        Self { stats_receiver }
    }

    pub fn stats(&mut self) -> CronReportStats {
        self.stats_receiver.borrow_and_update().clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronReportStats {
    pub messages: Vec<WsServerMessage>,
    pub window_size: usize,
    pub used_window_size: usize,
    pub queue_size: usize,
    pub period: Duration,
}

impl CronReportStats {
    pub fn new(
        messages: Vec<WsServerMessage>,
        window_size: usize,
        used_window_size: usize,
        queue_size: usize,
        period: Duration,
    ) -> Self {
        Self {
            messages,
            window_size,
            used_window_size,
            queue_size,
            period,
        }
    }
}
impl Default for CronReportStats {
    fn default() -> Self {
        Self::new(vec![], 0, 0, 0, Duration::from_secs(0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CronReportSettings {
    pub period: Option<Duration>,
    pub messages: Option<Vec<WsServerMessage>>,
    pub window_size: Option<usize>,
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
