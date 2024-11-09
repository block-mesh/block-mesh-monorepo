use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_manager_database_domain::domain::ws_bulk_daily_stats::ws_bulk_daily_stats;
use block_mesh_manager_database_domain::domain::ws_bulk_uptime::ws_bulk_uptime;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_loop", skip_all)]
pub async fn ws_bulk_loop(pool: PgPool, broadcaster: Broadcaster) -> anyhow::Result<()> {
    let base_msg_sleep = Duration::from_millis(
        env::var("WS_BULK_LOOP_SLEEP")
            .unwrap_or("60000".to_string())
            .parse()?,
    );
    let mut prev_time = Utc::now();

    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let diff = Utc::now() - prev_time;
            let user_ids: Vec<Uuid> = broadcaster.queue.lock().await.iter().map(|i| i.0).collect();
            let _ = ws_bulk_daily_stats(&mut transaction, &user_ids).await;
            let _ = ws_bulk_uptime(&mut transaction, &user_ids, diff.num_seconds() as f64).await;
            let _ = commit_txn(transaction).await;
        }
        prev_time = Utc::now();
        tokio::time::sleep(base_msg_sleep).await;
    }
}
