use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkIpData {
    user_id: Uuid,
    ip_id: Uuid,
    ip: String,
}

#[tracing::instrument(name = "ip_address_and_users_ip_bulk_query", skip_all, err)]
pub async fn ip_address_and_users_ip_bulk_query(
    pool: &PgPool,
    calls: HashMap<Uuid, String>,
) -> anyhow::Result<()> {
    if calls.is_empty() {
        return Ok(());
    }
    let now = Utc::now();
    let mut bulk_data: HashMap<(Uuid, Uuid), BulkIpData> = HashMap::new();
    let mut reverse_calls: HashMap<String, Uuid> = HashMap::new();
    calls.iter().for_each(|(id, ip)| {
        reverse_calls.insert(ip.clone(), *id);
    });
    let values: Vec<String> = reverse_calls
        .keys()
        .map(|ip| {
            format!(
                "(gen_random_uuid(), '{}', '{}'::timestamptz, false)",
                ip,
                now.to_rfc3339(),
            )
        })
        .collect();
    let value_str = values.join(",");
    let query = format!(
        // r#"SELECT id, ip from ip_addresses where ip in ({})"#,
        r#"INSERT
           INTO ip_addresses
           (id, ip, created_at, enriched)
           VALUES {}
           ON CONFLICT (ip) DO NOTHING
           RETURNING id, ip
        "#,
        value_str
    );
    let mut transaction = create_txn(pool).await?;
    let rows = sqlx::query(&query)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|e| {
            tracing::error!(
                "ip_address_and_users_ip_bulk_query failed to execute query size: {} , with error {:?}",
                values.len(),
                e
            );
            e
        })?;
    for row in rows {
        let ip_id = row.get::<Uuid, _>("id");
        let ip = row.get::<&str, _>("ip");
        if let Some(user_id) = reverse_calls.get(ip) {
            bulk_data.insert(
                (*user_id, ip_id),
                BulkIpData {
                    ip_id,
                    ip: ip.to_string(),
                    user_id: *user_id,
                },
            );
        }
    }
    let values: Vec<String> = bulk_data
        .values()
        .map(|i| {
            format!(
                "(gen_random_uuid(),'{}'::uuid, '{}'::uuid, '{}'::timestamptz, '{}'::timestamptz)",
                i.user_id,
                i.ip_id,
                now.to_rfc3339(),
                now.to_rfc3339()
            )
        })
        .collect();
    let value_str = values.join(",");
    let query = format!(
        r#"
            INSERT INTO users_ip (id, user_id, ip_id, created_at, updated_at)
            VALUES {}
            ON CONFLICT (user_id, ip_id) DO NOTHING
        "#,
        value_str
    );
    let _ = sqlx::query(&query)
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            tracing::error!("ip_address_and_users_ip_bulk_query failed to execute query size: {} , with error {:?}", values.len(), e);
            e
        })?;
    commit_txn(transaction).await?;
    Ok(())
}

#[tracing::instrument(name = "users_ip_aggregator", skip_all, err)]
pub async fn users_ip_aggregator(
    joiner_tx: Sender<JoinHandle<()>>,
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(DBMessage::UsersIpMessage(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
                    calls.insert(message.id, message.ip);
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        let calls_clone = calls.clone();
                        let poll_clone = pool.clone();
                        let handle = tokio::spawn(async move {
                            let _ =
                                ip_address_and_users_ip_bulk_query(&poll_clone, calls_clone).await;
                        });
                        let _ = joiner_tx.send_async(handle).await;
                        count = 0;
                        calls.clear();
                    }
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("users_ip_aggregator error recv: {:?}", e);
                    return Err(anyhow!("users_ip_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("users_ip_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
