use crate::domain::aggregate::{Aggregate, AggregateTmp};
use block_mesh_common::rand::init_rand;
use moka::future::Cache;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
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

    flush_aggregate_liveness_touches(pool, touches).await
}

async fn flush_aggregate_liveness_touches<I>(pool: &PgPool, touches: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = (Uuid, OffsetDateTime)>,
{
    let (user_ids, last_seen_ats): (Vec<Uuid>, Vec<OffsetDateTime>) = touches.into_iter().unzip();
    if user_ids.is_empty() {
        return Ok(0);
    }

    let result = sqlx::query!(
        r#"
UPDATE aggregates_uptime a
SET updated_at = v.last_seen_at
FROM unnest($1::uuid[], $2::timestamptz[]) AS v(user_id, last_seen_at)
WHERE a.user_id = v.user_id
AND a.updated_at < v.last_seen_at
"#,
        &user_ids,
        &last_seen_ats
    )
    .execute(pool)
    .await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{PgPool, QueryBuilder};

    async fn flush_aggregate_liveness_touches_with_query_builder<I>(
        pool: &PgPool,
        touches: I,
    ) -> anyhow::Result<u64>
    where
        I: IntoIterator<Item = (Uuid, OffsetDateTime)>,
    {
        let mut query_builder = build_flush_aggregate_liveness_query_with_query_builder(touches);
        let result = query_builder.build().execute(pool).await?;
        Ok(result.rows_affected())
    }

    fn build_flush_aggregate_liveness_query_with_query_builder<I>(
        touches: I,
    ) -> QueryBuilder<'static, Postgres>
    where
        I: IntoIterator<Item = (Uuid, OffsetDateTime)>,
    {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
        UPDATE aggregates_uptime a
        SET updated_at = v.last_seen_at
        FROM (
        "#,
        );
        query_builder.push_values(touches, |mut row, (user_id, last_seen_at)| {
            row.push_bind(user_id).push_bind(last_seen_at);
        });
        query_builder.push(
            r#"
        ) AS v(user_id, last_seen_at)
        WHERE a.user_id = v.user_id
        AND a.updated_at < v.last_seen_at
        "#,
        );
        query_builder
    }

    #[test]
    fn aggregate_liveness_query_uses_valid_values_tuples() {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let query_builder = build_flush_aggregate_liveness_query_with_query_builder(vec![
            (Uuid::nil(), now),
            (Uuid::nil(), now),
        ]);

        let sql = query_builder.sql();

        assert!(sql.contains("FROM (\n        VALUES ($1, $2), ($3, $4)"));
        assert!(!sql.contains("VALUES\n        (,$1"));
        assert!(!sql.contains("VALUES (,"));
    }

    #[tokio::test]
    async fn macro_liveness_update_matches_query_builder_implementation() -> anyhow::Result<()> {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            eprintln!("skipping database comparison test because DATABASE_URL is not set");
            return Ok(());
        };

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;
        sqlx::query(
            r#"
CREATE TEMP TABLE aggregates_uptime (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL UNIQUE,
    updated_at timestamptz NOT NULL
) ON COMMIT PRESERVE ROWS
"#,
        )
        .execute(&pool)
        .await?;

        let first_user_id = Uuid::new_v4();
        let second_user_id = Uuid::new_v4();
        let untouched_user_id = Uuid::new_v4();
        let initial_time = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let newer_time = OffsetDateTime::from_unix_timestamp(1_700_000_100).unwrap();
        let older_time = OffsetDateTime::from_unix_timestamp(1_699_999_900).unwrap();
        let touches = vec![(first_user_id, newer_time), (second_user_id, older_time)];

        seed_aggregate_liveness_rows(
            &pool,
            first_user_id,
            second_user_id,
            untouched_user_id,
            initial_time,
        )
        .await?;
        let query_builder_rows =
            flush_aggregate_liveness_touches_with_query_builder(&pool, touches.clone()).await?;
        let query_builder_state = fetch_aggregate_liveness_rows(&pool).await?;

        reset_aggregate_liveness_rows(
            &pool,
            first_user_id,
            second_user_id,
            untouched_user_id,
            initial_time,
        )
        .await?;
        let macro_rows = flush_aggregate_liveness_touches(&pool, touches).await?;
        let macro_state = fetch_aggregate_liveness_rows(&pool).await?;

        assert_eq!(query_builder_rows, macro_rows);
        assert_eq!(query_builder_state, macro_state);
        assert_eq!(macro_rows, 1);

        Ok(())
    }

    async fn seed_aggregate_liveness_rows(
        pool: &PgPool,
        first_user_id: Uuid,
        second_user_id: Uuid,
        untouched_user_id: Uuid,
        initial_time: OffsetDateTime,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
INSERT INTO aggregates_uptime (id, user_id, updated_at)
VALUES ($1, $2, $3), ($4, $5, $6), ($7, $8, $9)
"#,
        )
        .bind(Uuid::new_v4())
        .bind(first_user_id)
        .bind(initial_time)
        .bind(Uuid::new_v4())
        .bind(second_user_id)
        .bind(initial_time)
        .bind(Uuid::new_v4())
        .bind(untouched_user_id)
        .bind(initial_time)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn reset_aggregate_liveness_rows(
        pool: &PgPool,
        first_user_id: Uuid,
        second_user_id: Uuid,
        untouched_user_id: Uuid,
        initial_time: OffsetDateTime,
    ) -> anyhow::Result<()> {
        sqlx::query("TRUNCATE aggregates_uptime")
            .execute(pool)
            .await?;
        seed_aggregate_liveness_rows(
            pool,
            first_user_id,
            second_user_id,
            untouched_user_id,
            initial_time,
        )
        .await
    }

    async fn fetch_aggregate_liveness_rows(
        pool: &PgPool,
    ) -> anyhow::Result<Vec<(Uuid, OffsetDateTime)>> {
        let rows = sqlx::query_as::<_, (Uuid, OffsetDateTime)>(
            r#"
SELECT user_id, updated_at
FROM aggregates_uptime
ORDER BY user_id
"#,
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}
