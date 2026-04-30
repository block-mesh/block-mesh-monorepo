use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::{NaiveDate, Utc};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde_json::Value;
use sqlx::postgres::PgQueryResult;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashSet;
use std::env;
use std::sync::LazyLock;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uuid::Uuid;

static DAILY_STATS_ATTEMPTED_USERS: LazyLock<Mutex<(NaiveDate, HashSet<Uuid>)>> =
    LazyLock::new(|| Mutex::new((Utc::now().date_naive(), HashSet::new())));

async fn is_daily_stat_attempted_today(user_id: &Uuid) -> bool {
    let today = Utc::now().date_naive();
    let mut attempted = DAILY_STATS_ATTEMPTED_USERS.lock().await;
    if attempted.0 != today {
        attempted.0 = today;
        attempted.1.clear();
        return false;
    }
    attempted.1.contains(user_id)
}

async fn mark_daily_stats_attempted_today(user_ids: &[Uuid]) {
    let today = Utc::now().date_naive();
    let mut attempted = DAILY_STATS_ATTEMPTED_USERS.lock().await;
    if attempted.0 != today {
        attempted.0 = today;
        attempted.1.clear();
    }
    attempted.1.extend(user_ids.iter().copied());
}

#[tracing::instrument(name = "create_daily_stats_bulk_insert", skip_all, err)]
async fn create_daily_stats_bulk_insert(
    transaction: &mut Transaction<'_, Postgres>,
    user_ids: &[Uuid],
) -> anyhow::Result<u64> {
    if user_ids.is_empty() {
        return Ok(0);
    }

    let result: PgQueryResult = sqlx::query!(
        r#"
        WITH input AS (
            SELECT DISTINCT user_id
            FROM UNNEST($1::uuid[]) AS t(user_id)
        )
        INSERT INTO daily_stats (id, user_id, status, tasks_count, day, created_at, uptime, updated_at)
        SELECT
            gen_random_uuid(),
            u.id,
            'OnGoing',
            0,
            CURRENT_DATE,
            NOW(),
            0,
            NOW()
        FROM users u
        JOIN input ON input.user_id = u.id
        ON CONFLICT (status, user_id, day) DO NOTHING
        "#,
        user_ids
    )
    .execute(&mut **transaction)
    .await?;
    Ok(result.rows_affected())
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
                    if !is_daily_stat_attempted_today(&message.user_id).await {
                        calls.insert(message.user_id);
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
                            if save_to_db && !calls_clone.is_empty() {
                                if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                    let user_ids: Vec<Uuid> = calls_clone.into_iter().collect();
                                    match create_daily_stats_bulk_insert(
                                        &mut transaction,
                                        &user_ids,
                                    )
                                    .await
                                    {
                                        Ok(_) => match commit_txn(transaction).await {
                                            Ok(_) => {
                                                mark_daily_stats_attempted_today(&user_ids).await;
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "create_daily_stats_bulk_insert failed to commit query size: {} , with error {:?}",
                                                    count,
                                                    e
                                                );
                                            }
                                        },
                                        Err(e) => {
                                            tracing::error!(
                                                "create_daily_stats_bulk_insert failed to execute query size: {} , with error {:?}",
                                                count,
                                                e
                                            );
                                        }
                                    }
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
