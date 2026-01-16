use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_manager_database_domain::domain::ws_bulk_create_daily_stats::ws_bulk_create_daily_stats;
use block_mesh_manager_database_domain::domain::ws_bulk_daily_stats::ws_bulk_daily_stats;
#[allow(unused_imports)]
use block_mesh_manager_database_domain::domain::ws_bulk_uptime::ws_bulk_uptime;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use num_traits::abs;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_bulk_loop", skip_all)]
pub async fn ws_bulk_loop(pool: PgPool, broadcaster: Arc<Broadcaster>) -> anyhow::Result<()> {
    let enable_ws_bulk_loop = env::var("WS_BULK_LOOP_ENABLE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let base_msg_sleep = Duration::from_millis(
        env::var("WS_BULK_LOOP_SLEEP")
            .unwrap_or("60000".to_string())
            .parse()
            .unwrap_or(60_000),
    );
    let chunk_size = env::var("WS_BULK_CHUNK_SIZE")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);
    let mut prev_time = Utc::now();
    loop {
        if enable_ws_bulk_loop {
            tracing::info!("ws_bulk_loop starting");
            let user_ids: Vec<Uuid> = broadcaster.user_ids.iter().map(|i| *i.key()).collect();
            tracing::info!("ws_bulk_loop starting user_ids: {}", user_ids.len());
            for chunk in user_ids.chunks(chunk_size) {
                let diff = Utc::now() - prev_time;
                let sec_diff = abs(diff.num_seconds());
                if let Ok(mut transaction) = create_txn(&pool).await {
                    let _ = ws_bulk_create_daily_stats(&mut transaction, chunk)
                        .await
                        .map_err(|e| tracing::error!("ws_bulk_create_daily_stats error: {:?}", e));
                    let _ = commit_txn(transaction).await;
                }
                if let Ok(mut transaction) = create_txn(&pool).await {
                    let _ = ws_bulk_daily_stats(&mut transaction, chunk, sec_diff as f64)
                        .await
                        .map_err(|e| tracing::error!("ws_bulk_daily_stats error: {:?}", e));
                    let _ = commit_txn(transaction).await;
                }
                if let Ok(mut transaction) = create_txn(&pool).await {
                    let _ = ws_bulk_uptime(&mut transaction, chunk, sec_diff as f64)
                        .await
                        .map_err(|e| tracing::error!("ws_bulk_uptime error: {:?}", e));
                    let _ = commit_txn(transaction).await;
                }
            }
            prev_time = Utc::now();
        }
        tokio::time::sleep(base_msg_sleep).await;
    }
}
