use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use std::env;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use uuid::Uuid;

type AggregateKey = (Uuid, String);

fn value_as_f64(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_i64().map(|value| value as f64))
        .or_else(|| value.as_u64().map(|value| value as f64))
}

#[tracing::instrument(name = "add_to_aggregates_bulk_update", skip_all, err)]
async fn add_to_aggregates_bulk_update(
    transaction: &mut Transaction<'_, Postgres>,
    calls: HashMap<AggregateKey, f64>,
) -> anyhow::Result<u64> {
    if calls.is_empty() {
        return Ok(0);
    }
    let now = Utc::now();
    let mut user_ids = Vec::with_capacity(calls.len());
    let mut names = Vec::with_capacity(calls.len());
    let mut values = Vec::with_capacity(calls.len());
    let mut updated_ats = Vec::with_capacity(calls.len());

    for ((user_id, name), value) in calls {
        user_ids.push(user_id);
        names.push(name);
        values.push(value);
        updated_ats.push(now);
    }

    let result = sqlx::query!(
        r#"
        WITH updates AS (
            SELECT *
            FROM UNNEST(
                $1::uuid[],
                $2::text[],
                $3::double precision[],
                $4::timestamptz[]
            ) AS t(user_id, name, value, updated_at)
        )
        UPDATE aggregates
        SET
            value = to_jsonb((COALESCE(NULLIF(aggregates.value, 'null'), '0')::text)::double precision + updates.value),
            updated_at = updates.updated_at
        FROM updates
        WHERE
            aggregates.user_id = updates.user_id
            AND aggregates.name = updates.name
        "#,
        &user_ids,
        &names,
        &values,
        &updated_ats
    )
    .execute(&mut **transaction)
    .await?;
    Ok(result.rows_affected())
}

#[tracing::instrument(name = "add_to_aggregates_aggregator", skip_all, err)]
pub async fn add_to_aggregates_aggregator(
    joiner_tx: Sender<JoinHandle<()>>,
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<AggregateKey, f64> = HashMap::new();
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
                    if let Some(value) = value_as_f64(&message.value) {
                        *calls.entry((message.user_id, message.name)).or_default() += value;
                    } else {
                        tracing::error!(
                            "add_to_aggregates_aggregator received non numeric value {:?}",
                            message.value
                        );
                    }
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
                                    let _ = add_to_aggregates_bulk_update(
                                        &mut transaction,
                                        calls_clone,
                                    )
                                    .await
                                    .map_err(|e| {
                                        tracing::error!(
                                            "add_to_aggregates_bulk_update failed to execute query size: {} , with error {:?}",
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
