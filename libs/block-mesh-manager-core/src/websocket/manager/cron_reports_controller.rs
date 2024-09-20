use block_mesh_common::interfaces::ws_api::WsServerMessage;
use serde::{Deserialize, Serialize};
use std::time::Duration;
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
