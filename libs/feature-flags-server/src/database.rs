use crate::error::Error;
use block_mesh_common::solana::get_block_time;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use database_utils::utils::option_uuid::OptionUuid;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use sqlx::Postgres;
use sqlx::{PgPool, Transaction};
use std::sync::Arc;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Flag {
    pub id: Uuid,
    pub name: String,
    pub value: Value,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct FlagTmp {
    pub id: OptionUuid,
    pub name: Option<String>,
    pub value: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
}

#[tracing::instrument(name = "get_flag", skip_all)]
pub async fn get_flag(
    transaction: &mut Transaction<'_, Postgres>,
    name: &str,
) -> anyhow::Result<Flag> {
    let flag = sqlx::query_as!(
        Flag,
        r#"
        SELECT id, name, value, created_at
        FROM flags
        WHERE name = $1
        LIMIT 1
        "#,
        name
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(flag)
}

#[tracing::instrument(name = "get_flags", skip_all)]
pub async fn get_flags(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<Flag>> {
    let flags = sqlx::query_as!(
        Flag,
        r#"
        SELECT id, name, value, created_at
        FROM flags
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(flags)
}

#[tracing::instrument(name = "load_flags", skip_all)]
pub async fn load_flags(
    flags_cache: Arc<DashMap<String, Value>>,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(pool).await.map_err(Error::from)?;
    let flags = get_flags(&mut transaction).await?;
    for flag in flags {
        flags_cache.insert(flag.name, flag.value.clone());
    }
    commit_txn(transaction).await?;
    Ok(())
}

#[tracing::instrument(name = "load_flags_cron", skip_all)]
pub async fn load_flags_cron(
    flags_cache: Arc<DashMap<String, Value>>,
    pool: PgPool,
) -> anyhow::Result<()> {
    let sleep = std::time::Duration::from_millis(60_000);
    loop {
        load_flags(flags_cache.clone(), &pool).await?;
        let block_time = get_block_time().await;
        get_or_create_flag(&pool, "block_time", Value::Number(Number::from(block_time))).await?;
        tokio::time::sleep(sleep).await;
    }
}

pub async fn get_or_create_flag(pool: &PgPool, name: &str, value: Value) -> anyhow::Result<Flag> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let flag = sqlx::query_as!(
        FlagTmp,
        r#"
        WITH inserted AS (
            INSERT INTO flags
            (id, name, value, created_at)
            VALUES
            ($1, $2, $3, $4)
            ON CONFLICT (name)
            DO UPDATE SET created_at = now()
            RETURNING id, name, value, created_at
        )
        SELECT id, name, value, created_at FROM inserted
        UNION
        SELECT id, name, value, created_at FROM flags WHERE name = $2 AND NOT EXISTS (SELECT 1 FROM inserted)
        "#,
        id,
        name,
        value,
        now
    )
    .fetch_one(pool)
    .await?;
    Ok(Flag {
        id: flag.id.unwrap_or_default(),
        name: flag.name.unwrap_or_default(),
        value: flag.value.unwrap_or_default(),
        created_at: flag.created_at,
    })
}

pub async fn create_flag(pool: &PgPool, name: &str, value: Value) -> anyhow::Result<()> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO flags
        (id, name, value, created_at)
        VALUES
        ($1, $2, $3, $4)
        "#,
        id,
        name,
        value,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn pre_populate_db(pool: &PgPool) -> anyhow::Result<()> {
    create_flag(
        pool,
        "enrich_ip_and_cleanup_in_background",
        Value::Bool(false),
    )
    .await?;
    create_flag(
        pool,
        "polling_interval",
        Value::Number(Number::from(300_000)),
    )
    .await?;
    create_flag(
        pool,
        "report_uptime_daily_stats_via_channel",
        Value::Bool(true),
    )
    .await?;

    create_flag(
        pool,
        "minimal_version",
        Value::String("0.0.515".to_string()),
    )
    .await?;
    create_flag(
        pool,
        "current_version",
        Value::String(env!("CARGO_PKG_VERSION").to_string()),
    )
    .await?;
    create_flag(pool, "send_cleanup_to_rayon", Value::Bool(true)).await?;
    create_flag(pool, "send_to_worker", Value::Bool(true)).await?;
    create_flag(pool, "submit_bandwidth_run_background", Value::Bool(false)).await?;
    create_flag(pool, "submit_bandwidth_via_channel", Value::Bool(true)).await?;
    create_flag(pool, "touch_users_ip", Value::Bool(true)).await?;
    create_flag(pool, "tx_analytics_agg", Value::Bool(true)).await?;
    create_flag(pool, "use_websocket", Value::Bool(true)).await?;
    create_flag(
        pool,
        "use_websocket_percent",
        Value::Number(Number::from(50)),
    )
    .await?;
    Ok(())
}
