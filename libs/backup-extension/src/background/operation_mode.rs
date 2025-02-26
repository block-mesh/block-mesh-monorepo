use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum OperationMode {
    Http,
    WebSocket,
}
