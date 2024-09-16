use block_mesh_common::interfaces::ws_api::WsServerMessage;
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CronReportSettings {
    pub period: Option<Duration>,
    pub messages: Option<Vec<WsServerMessage>>,
    pub window_size: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CronReportAggregateEntry {
    pub period: Duration,
    pub messages: Vec<WsServerMessage>,
    pub window_size: usize,
    pub used_window_size: usize,
    pub queue_size: usize,
}

impl Default for CronReportAggregateEntry {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(10),
            vec![
                WsServerMessage::RequestUptimeReport,
                WsServerMessage::RequestBandwidthReport,
            ],
            10,
            0,
            0,
        )
    }
}

impl CronReportAggregateEntry {
    pub fn new(
        period: Duration,
        messages: Vec<WsServerMessage>,
        window_size: usize,
        used_window_size: usize,
        queue_size: usize,
    ) -> Self {
        Self {
            period,
            messages,
            window_size,
            used_window_size,
            queue_size,
        }
    }
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
