pub mod broadcaster;
pub mod task_scheduler;

use crate::websocket::manager::broadcaster::Broadcaster;
use crate::websocket::manager::task_scheduler::TaskScheduler;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct WebSocketManager {
    pub broadcaster: Arc<Broadcaster>,
    pub task_scheduler: Arc<TaskScheduler<WsServerMessage>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            broadcaster: Arc::new(Broadcaster::new()),
            task_scheduler: Arc::new(TaskScheduler::new()),
        }
    }
}
