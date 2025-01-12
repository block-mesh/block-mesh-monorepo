use crate::state::WsAppState;
use block_mesh_manager_database_domain::domain::twitter_task::TwitterTask;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tracing::instrument(name = "get_pending_twitter_tasks_loop", skip_all)]
pub async fn get_pending_twitter_tasks_loop(state: Arc<WsAppState>) -> Result<(), anyhow::Error> {
    let dur = Duration::from_millis(
        env::var("TWITTER_TASKS_SLEEP")
            .unwrap_or("5000".to_string())
            .parse()
            .unwrap_or(5000),
    );
    loop {
        if let Ok(mut transaction) = create_txn(&state.pool).await {
            if let Ok(tasks) = TwitterTask::get_pending_tasks(&mut transaction).await {
                let mut t = state.pending_twitter_tasks.write().await;
                tasks.into_iter().for_each(|i| {
                    t.insert(i.id, i);
                });
            }
            let _ = commit_txn(transaction).await;
        }
        sleep(dur).await;
    }
}
