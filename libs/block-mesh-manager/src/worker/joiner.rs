use tokio::task::JoinHandle;

#[tracing::instrument(name = "joiner", skip(rx))]
pub async fn joiner_loop(
    mut rx: tokio::sync::mpsc::Receiver<JoinHandle<()>>,
) -> Result<(), anyhow::Error> {
    while let Some(handle) = rx.recv().await {
        let _ = handle.await;
    }
    Ok(())
}
