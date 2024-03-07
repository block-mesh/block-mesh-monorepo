use crate::token_management::channels::ChannelMessage;
use tokio::sync::broadcast::Sender;

#[derive(Clone, Debug)]
pub struct AppState {
    pub tx: Sender<ChannelMessage>,
}
