use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "joiner", skip(join_handles))]
pub async fn joiner_loop(
    join_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    mut rx: tokio::sync::mpsc::Receiver<()>,
) -> Result<(), anyhow::Error> {
    while let Some(_) = rx.recv().await {
        while let Some(handle) = join_handles.lock().await.pop() {
            let _ = handle.await;
        }
    }
    Ok(())
}
