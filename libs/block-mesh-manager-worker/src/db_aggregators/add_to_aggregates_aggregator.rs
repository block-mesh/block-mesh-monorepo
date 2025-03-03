use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[tracing::instrument(name = "add_to_aggregates_create_bulk_query", skip_all)]
pub fn add_to_aggregates_create_bulk_query(calls: HashMap<Uuid, (String, Value)>) -> String {
    let now = Utc::now();
    let lock_values: Vec<String> = calls
        .iter()
        .map(|(user_id, value)| format!("('{}'::uuid, '{}')", user_id, value.0))
        .collect();
    let lock_values_str = lock_values.join(",");

    let update_values: Vec<String> = calls
        .iter()
        .map(|(user_id, value)| {
            format!(
                "('{}'::uuid, '{}'::jsonb, '{}'::timestamptz, '{}')",
                user_id,
                value.1,
                now.to_rfc3339(),
                value.0
            )
        })
        .collect();

    let update_values_str = update_values.join(",");
    format!(
        r#"
        WITH
        locked_rows (user_id, name) AS (
            SELECT user_id, name
            FROM aggregates
            WHERE (user_id, name) IN ( {lock_values_str} )
            FOR UPDATE SKIP LOCKED
        ),
        updates (user_id, value, updated_at, name) AS ( VALUES {update_values_str} )
        UPDATE aggregates
            SET
                value =  to_jsonb((COALESCE(NULLIF(aggregates.value, 'null'), '0')::text)::double precision + updates.value::double precision),
                updated_at = updates.updated_at
        FROM updates
        JOIN locked_rows ON locked_rows.user_id = updates.user_id AND locked_rows.name = updates.name
        WHERE
            aggregates.user_id = updates.user_id
            AND aggregates.name = updates.name
        "#
    )
}

#[tracing::instrument(name = "add_to_aggregates_aggregator", skip_all, err)]
pub async fn add_to_aggregates_aggregator(
    joiner_tx: Sender<JoinHandle<()>>,
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<Uuid, (String, Value)> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    let save_to_db = env::var("ADD_TO_AGGREGATOR_SAVE_TO_DB")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(DBMessage::AggregateAddToMessage(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
                    calls.insert(message.user_id, (message.name, message.value));
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        let calls_clone = calls.clone();
                        let poll_clone = pool.clone();
                        let handle = tokio::spawn(async move {
                            tracing::info!("add_to_aggregates_create_bulk_query starting txn");
                            if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                if save_to_db {
                                    let query = add_to_aggregates_create_bulk_query(calls_clone);
                                    let r = sqlx::query(&query)
                                        .execute(&mut *transaction)
                                        .await
                                        .map_err(|e| {
                                            tracing::error!(
                                                "add_to_aggregates_create_bulk_query failed to execute query size: {} , with error {:?}",
                                                count,
                                                e
                                            );
                                        });
                                    if let Ok(r) = r {
                                        tracing::info!(
                                            "add_to_aggregates_create_bulk_query rows_affected : {}",
                                            r.rows_affected()
                                        );
                                    }
                                    let _ = commit_txn(transaction).await;
                                    tracing::info!(
                                        "add_to_aggregates_create_bulk_query finished txn"
                                    );
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
                    tracing::error!("add_to_aggregates_aggregator error recv: {:?}", e);
                    return Err(anyhow!("add_to_aggregates_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("add_to_aggregates_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
