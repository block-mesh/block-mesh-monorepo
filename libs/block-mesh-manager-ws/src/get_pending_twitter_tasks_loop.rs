use crate::state::WsAppState;
use block_mesh_manager_database_domain::domain::twitter_task::TwitterTask;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::cmp::min;
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

    let assign_limit = env::var("TWITTER_ASSIGN_LIMIT")
        .unwrap_or("10000".to_string())
        .parse()
        .unwrap_or(10_000usize);

    let twitter_tasks = env::var("TWITTER_TASKS")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);

    loop {
        if twitter_tasks {
            if let Ok(mut transaction) = create_txn(&state.pool).await {
                if let Ok(tasks) = TwitterTask::get_pending_tasks(&mut transaction).await {
                    let mut t = state.pending_twitter_tasks.write().await;
                    tasks.into_iter().for_each(|i| {
                        t.insert(i.id, i);
                    });
                }
                state.clear_tasks().await;
                let _ = commit_txn(transaction).await;
                let limit = min(assign_limit, state.workers.read().await.len());
                for _ in 0..limit {
                    state.assign_task().await;
                    sleep(Duration::from_millis(10)).await;
                }
            }
        }
        sleep(dur).await;
    }
}
