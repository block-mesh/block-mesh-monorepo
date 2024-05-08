use block_mesh_common::cli::CliArgs;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct AppState {
    pub cli_args: CliArgs,
    pub tx: Sender<ChannelMessage>,
    pub rx: Receiver<ChannelMessage>,
}

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    RestartTask,
}
