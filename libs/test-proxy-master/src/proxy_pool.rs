use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};

#[derive(Debug, Clone, Default)]
pub struct ProxyPool {
    pool: Arc<Mutex<Vec<TcpStream>>>,
}

impl ProxyPool {
    pub async fn put(&self, stream: TcpStream) {
        self.pool.lock().await.push(stream);
    }

    pub async fn get(&self) -> Option<TcpStream> {
        let mut lock = self.pool.lock().await;

        // We have all proxy connection now, so we can pick any of them by arbitrary condition

        // Just pop the last one for example
        lock.pop()
    }
}
