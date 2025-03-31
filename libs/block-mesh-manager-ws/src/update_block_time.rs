use crate::state::WsAppState;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tracing::instrument(name = "update_block_time_loop", skip_all)]
pub async fn update_block_time_loop(state: Arc<WsAppState>) -> Result<(), anyhow::Error> {
    let sleep = env::var("UPDATE_BLOCK_TIME_LOOP_SLEEP")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(60000);
    loop {
        state.update_block_time().await;
        tokio::time::sleep(Duration::from_millis(sleep)).await;
    }
}
