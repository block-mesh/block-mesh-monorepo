use block_mesh_common::app_config::AppConfig;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct AppState {
    pub config: AppConfig,
    pub tx: Sender<ChannelMessage>,
    pub rx: Receiver<ChannelMessage>,
}

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    RestartTask,
}
