use block_mesh_common::interfaces::db_messages::DBMessage;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use flume::Receiver;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;

#[tracing::instrument(name = "collect_messages", skip_all)]
pub async fn collect_messages(
    rx: Receiver<DBMessage>,
    channel_pool: PgPool,
    agg_size: i32,
    time_limit: i64,
) -> anyhow::Result<()> {
    let mut messages: Vec<DBMessage> = Vec::with_capacity(agg_size as usize);
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(msg) = rx.recv_async().await {
        messages.push(msg);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > time_limit || count >= agg_size;
        prev = Utc::now();
        if run {
            let _ = notify_worker(&channel_pool, &messages).await;
            messages.clear();
            count = 0;
        }
    }
    Ok(())
}
