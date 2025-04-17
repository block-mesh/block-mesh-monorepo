use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashSet;
use std::env;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[tracing::instrument(name = "create_daily_stats_create_bulk_query", skip_all)]
pub fn create_daily_stats_create_bulk_query(calls: HashSet<Uuid>) -> String {
    let values: Vec<String> = calls
        .iter()
        .map(|user_id| format!("('{}'::uuid)", user_id))
        .collect();
    let value_str = values.join(",");
    format!(
        r#"
INSERT INTO daily_stats (id, user_id, status, tasks_count, day, created_at, uptime, updated_at) (
	SELECT
		gen_random_uuid (),
		u.id,
		'OnGoing',
		0,
		CURRENT_DATE,
		NOW(),
		0,
		NOW()
	FROM users u
	WHERE u.id IN ({value_str})
	) ON CONFLICT (status, user_id, day)
	DO NOTHING
    "#
    )
}

#[tracing::instrument(name = "create_daily_stats_aggregator", skip_all, err)]
pub async fn create_daily_stats_aggregator(
    joiner_tx: Sender<JoinHandle<()>>,
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashSet<Uuid> = HashSet::new();
    let mut count = 0;
    let mut prev = Utc::now();
    let save_to_db = env::var("CREATE_STATS_AGGREGATOR_SAVE_TO_DB")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(DBMessage::CreateDailyStatMessage(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
                    calls.insert(message.user_id);
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        let calls_clone = calls.clone();
                        let poll_clone = pool.clone();
                        let handle = tokio::spawn(async move {
                            if save_to_db {
                                if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                    let query = create_daily_stats_create_bulk_query(calls_clone);
                                    let _ = sqlx::query(&query)
                                        .execute(&mut *transaction)
                                        .await
                                        .map_err(|e| {
                                            tracing::error!(
                                                "create_daily_stats_create_bulk_query failed to execute query size: {} , with error {:?}",
                                                count,
                                                e
                                            );
                                        });
                                    let _ = commit_txn(transaction).await;
                                }
                            }
                        });
                        let _ = joiner_tx.send_async(handle).await;
                        count = 0;
                        calls.clear();
                    }
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("create_daily_stats_aggregator error recv: {:?}", e);
                    return Err(anyhow!("create_daily_stats_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("create_daily_stats_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
