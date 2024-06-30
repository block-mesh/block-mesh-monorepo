use std::fmt::{Display, Formatter};
use tokio::sync::broadcast::{Receiver, Sender};

use block_mesh_common::app_config::AppConfig;

pub struct AppState {
    pub config: AppConfig,
    pub tx: Sender<ChannelMessage>,
    pub rx: Receiver<ChannelMessage>,
    pub ore_pid: Option<u32>,
    pub node_handle: Option<tauri::async_runtime::JoinHandle<()>>,
    pub uptime_handle: Option<tauri::async_runtime::JoinHandle<()>>,
    pub task_puller: Option<tauri::async_runtime::JoinHandle<()>>,
}

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    StartOre,
    ShutdownOre,
    StartUptime,
    ShutdownUptime,
    StartTaskPull,
    ShutdownTaskPull,
    StartNode,
    ShutdownNode,
    RestartTask,
}

impl Display for ChannelMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ChannelMessage::StartOre => f.write_str("StartOre"),
            ChannelMessage::ShutdownOre => f.write_str("ShutdownOre"),
            ChannelMessage::StartUptime => f.write_str("StartUptime"),
            ChannelMessage::ShutdownUptime => f.write_str("ShutdownUptime"),
            ChannelMessage::StartTaskPull => f.write_str("StartTaskPull"),
            ChannelMessage::ShutdownTaskPull => f.write_str("ShutdownTaskPull"),
            ChannelMessage::StartNode => f.write_str("StartNode"),
            ChannelMessage::ShutdownNode => f.write_str("ShutdownNode"),
            ChannelMessage::RestartTask => f.write_str("RestartTask"),
        }
    }
}
