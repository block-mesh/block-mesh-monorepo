use hyper::upgrade::Upgraded;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct ProxyPool {
    pool: Arc<Mutex<Vec<Upgraded>>>,
}

impl ProxyPool {
    #[tracing::instrument(name = "ProxyPool#put")]
    pub async fn put(&self, stream: Upgraded) {
        self.pool.lock().await.push(stream);
    }

    #[tracing::instrument(name = "ProxyPool#get")]
    pub async fn get(&self) -> Option<Upgraded> {
        let mut lock = self.pool.lock().await;

        // We have all proxy connection now, so we can pick any of them by arbitrary condition

        // Just pop the last one for example
        lock.pop()
    }
}
