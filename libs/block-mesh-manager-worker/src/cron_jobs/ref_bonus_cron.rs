use block_mesh_manager_database_domain::domain::daily_stat_background_job::DailyStatsBackgroundJob;
use flume::Sender;
use sqlx::PgPool;
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use time::Date;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[tracing::instrument(name = "ref_bonus_cron", skip_all)]
pub async fn ref_bonus_cron(
    pool: PgPool,
    joiner_tx: Sender<JoinHandle<()>>,
    queue: Arc<RwLock<HashSet<(Uuid, Uuid, Date)>>>,
) -> Result<(), anyhow::Error> {
    let enable = env::var("REF_BONUS_CRON_ENABLE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let zero_duration = Duration::from_millis(1_000);
    let non_zero_duration = Duration::from_millis(100);
    loop {
        if enable {
            if let Some(item) = queue.read().await.iter().next() {
                let poll_clone = pool.clone();
                let item_clone = *item;
                let handle = tokio::spawn(async move {
                    let _ = DailyStatsBackgroundJob::process_job(
                        poll_clone,
                        item_clone.0,
                        item_clone.1,
                        item_clone.2,
                    )
                    .await;
                });
                let _ = joiner_tx.send_async(handle).await;
                queue.write().await.remove(item);
            }
            let size = queue.read().await.len();
            if size == 0 {
                tokio::time::sleep(zero_duration).await;
            } else {
                tokio::time::sleep(non_zero_duration).await;
            }
        } else {
            tokio::time::sleep(zero_duration).await;
        }
    }
}
