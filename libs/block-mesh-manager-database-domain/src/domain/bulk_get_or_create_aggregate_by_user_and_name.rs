use crate::domain::aggregate::{Aggregate, AggregateTmp};
use block_mesh_common::rand::init_rand;
use moka::future::Cache;
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use std::collections::HashMap;
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

const DEFAULT_AGGREGATE_CACHE_TTL_SECONDS: u64 = 60;
const DEFAULT_AGGREGATE_LIVENESS_FLUSH_MS: u64 = 60_000;

fn aggregate_cache_ttl_seconds() -> u64 {
    env::var("AGGREGATE_CACHE_TTL_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(DEFAULT_AGGREGATE_CACHE_TTL_SECONDS)
}

static AGGREGATE_CACHE: LazyLock<Cache<Uuid, Vec<Aggregate>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(50_000)
        .time_to_live(Duration::from_secs(aggregate_cache_ttl_seconds()))
        .build()
});

static AGGREGATE_LIVENESS_TOUCHES: LazyLock<Mutex<HashMap<Uuid, OffsetDateTime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub async fn get_aggregates_from_cache(user_id: &Uuid) -> Option<Vec<Aggregate>> {
    AGGREGATE_CACHE.get(user_id).await
}

#[tracing::instrument(name = "record_aggregate_liveness", skip_all)]
pub async fn record_aggregate_liveness(user_id: &Uuid) {
    let mut touches = AGGREGATE_LIVENESS_TOUCHES.lock().await;
    touches.insert(*user_id, OffsetDateTime::now_utc());
}

#[tracing::instrument(name = "flush_aggregate_liveness", skip_all, err)]
pub async fn flush_aggregate_liveness(pool: &PgPool) -> anyhow::Result<u64> {
    let touches = {
        let mut touches = AGGREGATE_LIVENESS_TOUCHES.lock().await;
        if touches.is_empty() {
            return Ok(0);
        }
        std::mem::take(&mut *touches)
    };

    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        UPDATE aggregates_uptime a
        SET updated_at = v.last_seen_at
        FROM (VALUES
        "#,
    );
    let mut separated = query_builder.separated(",");
    for (user_id, last_seen_at) in touches {
        separated
            .push("(")
            .push_bind(user_id)
            .push(",")
            .push_bind(last_seen_at)
            .push(")");
    }
    query_builder.push(
        r#"
        ) AS v(user_id, last_seen_at)
        WHERE a.user_id = v.user_id
        AND a.updated_at < v.last_seen_at
        "#,
    );

    let result = query_builder.build().execute(pool).await?;
    Ok(result.rows_affected())
}

#[tracing::instrument(name = "flush_aggregate_liveness_loop", skip_all, err)]
pub async fn flush_aggregate_liveness_loop(pool: PgPool) -> anyhow::Result<()> {
    let flush_ms = env::var("AGGREGATE_LIVENESS_FLUSH_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_AGGREGATE_LIVENESS_FLUSH_MS);
    let duration = Duration::from_millis(flush_ms);

    loop {
        tokio::time::sleep(duration).await;
        match flush_aggregate_liveness(&pool).await {
            Ok(rows_affected) if rows_affected > 0 => {
                tracing::info!(
                    "flushed aggregate liveness rows_affected = {}",
                    rows_affected
                );
            }
            Ok(_) => {}
            Err(error) => {
                tracing::error!("failed to flush aggregate liveness: {:?}", error);
            }
        }
    }
}

#[tracing::instrument(name = "bulk_get_or_create_aggregate_by_user_and_name_old", skip_all)]
pub async fn bulk_get_or_create_aggregate_by_user_and_name_old(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<Aggregate>> {
    let upload = Value::from(init_rand(1, 100));
    let download = Value::from(init_rand(1, 100));
    let latency = Value::from(init_rand(1, 100));
    let value = Value::Null;
    let aggregates: Vec<AggregateTmp> = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH input_data AS (
    -- Input data
    SELECT *
    FROM (
    VALUES
        (gen_random_uuid(), now(), now(), $1::uuid, 'Uptime', $2::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Download', $4::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Upload', $3::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Latency', $5::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Tasks', $2::jsonb)
    )
    AS t (id, created_at, updated_at, user_id, name, value)
),
extant AS (
	-- Existing records matching user_id and name
	SELECT id, created_at, user_id, name, value, updated_at
	FROM aggregates
	WHERE (user_id, name) IN(SELECT user_id, name FROM input_data)
),
inserted AS (
	-- Insert new records where they do not exist
	INSERT INTO aggregates (id, created_at, user_id, name, value, updated_at, dummy_updated_at)
    SELECT id, created_at, user_id, name, value, created_at, created_at
    FROM input_data
    WHERE NOT EXISTS (
        SELECT 1
		FROM extant e
		WHERE
			e.user_id = input_data.user_id
			AND e.name = input_data.name
	)
	RETURNING
		id,
		created_at,
		user_id,
		name,
		value,
		updated_at
)
-- Combine results from inserted and existing records
SELECT id, created_at, user_id, name, value, updated_at FROM inserted
UNION ALL
SELECT id, created_at, user_id, name, value, updated_at FROM extant;
"#,
        user_id,
        value,
        upload,
        download,
        latency
    )
    .fetch_all(&mut **transaction)
    .await?;
    let aggregates = aggregates
        .into_iter()
        .map(|aggregate| Aggregate {
            id: aggregate.id.expect("MISSING ID"),
            user_id: aggregate.user_id.expect("MISSING USER ID"),
            name: aggregate.name.expect("MISSING NAME").into(),
            value: aggregate.value.expect("MISSING VALUE"),
            created_at: aggregate.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
            updated_at: aggregate.updated_at.expect("MISSING TIMESTAMP UPDATED_AT"),
        })
        .collect();
    Ok(aggregates)
}

#[tracing::instrument(name = "bulk_get_or_create_aggregate_by_user_and_name", skip_all)]
pub async fn bulk_get_or_create_aggregate_by_user_and_name(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<Aggregate>> {
    if let Some(cached) = AGGREGATE_CACHE.get(user_id).await {
        return Ok(cached);
    }
    let zero = Value::from(0i32);
    let upload = Value::from(init_rand(1, 100));
    let download = Value::from(init_rand(1, 100));
    let latency = Value::from(init_rand(1, 100));
    let value = Value::Null;
    sqlx::query!(
        r#"
WITH input_data(id, created_at, updated_at, user_id, name, value) AS (
  VALUES
    (gen_random_uuid(), now(), now(), $1::uuid, 'Uptime',           $2::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'InteractiveExt',   $6::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Wootz',            $6::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Download',         $4::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Upload',           $3::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Latency',          $5::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Tasks',            $2::jsonb)
)
INSERT INTO aggregates (id, created_at, user_id, name, value, updated_at, dummy_updated_at)
SELECT id, created_at, user_id, name, value, created_at, created_at
FROM input_data
ON CONFLICT (user_id, name) DO NOTHING
"#,
        user_id,
        value,
        upload,
        download,
        latency,
        zero
    )
    .execute(&mut **transaction)
    .await?;
    let aggregates: Vec<AggregateTmp> = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH input_data(user_id, name) AS (
  VALUES
    ($1::uuid, 'Uptime'),
    ($1::uuid, 'InteractiveExt'),
    ($1::uuid, 'Wootz'),
    ($1::uuid, 'Download'),
    ($1::uuid, 'Upload'),
    ($1::uuid, 'Latency'),
    ($1::uuid, 'Tasks')
)
SELECT
    a.id AS "id?",
    a.created_at AS "created_at?",
    a.user_id AS "user_id?",
    a.name AS "name?",
    a.value AS "value?",
    a.updated_at AS "updated_at?"
FROM aggregates a
JOIN input_data i USING (user_id, name)
"#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    let aggregates: Vec<Aggregate> = aggregates
        .into_iter()
        .map(|aggregate| Aggregate {
            id: aggregate.id.expect("MISSING ID"),
            user_id: aggregate.user_id.expect("MISSING USER ID"),
            name: aggregate.name.expect("MISSING NAME").into(),
            value: aggregate.value.expect("MISSING VALUE"),
            created_at: aggregate.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
            updated_at: aggregate.updated_at.expect("MISSING TIMESTAMP UPDATED_AT"),
        })
        .collect();
    AGGREGATE_CACHE.insert(*user_id, aggregates.clone()).await;
    record_aggregate_liveness(user_id).await;
    Ok(aggregates)
}
