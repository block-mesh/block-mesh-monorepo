use block_mesh_common::app_config::{AppConfig, TaskStatus};
use tokio::sync::broadcast::{Receiver, Sender};

pub struct AppState {
    pub config: AppConfig,
    pub tx: Sender<ChannelMessage>,
    pub rx: Receiver<ChannelMessage>,
    pub ore_tx: tokio::sync::mpsc::Sender<TaskStatus>,
    pub ore_pid: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    RestartTask,
}
