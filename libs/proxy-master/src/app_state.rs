use crate::token_management::channels::{ChannelMessage, TokenManagerHashMap};
// use block_mesh_solana_client::manager::SolanaManager;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<ChannelMessage>,
    pub token_manager: Arc<RwLock<TokenManagerHashMap>>,
    // pub solana_manager: Arc<RwLock<SolanaManager>>,
}
