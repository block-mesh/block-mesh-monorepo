use std::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "tokio_joiner_loop", skip(rx))]
pub async fn tokio_joiner_loop(rx: Receiver<JoinHandle<()>>) -> Result<(), anyhow::Error> {
    while let Ok(handle) = rx.recv() {
        let _ = handle.await;
    }
    Ok(())
}
