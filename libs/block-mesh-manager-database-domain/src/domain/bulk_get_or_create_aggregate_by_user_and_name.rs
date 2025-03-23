use crate::domain::aggregate::{Aggregate, AggregateTmp};
use rand::Rng;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub fn init_rand() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=100);
    random_number
}

#[tracing::instrument(name = "bulk_get_or_create_aggregate_by_user_and_name_old", skip_all)]
pub async fn bulk_get_or_create_aggregate_by_user_and_name_old(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<Aggregate>> {
    let upload = Value::from(init_rand());
    let download = Value::from(init_rand());
    let latency = Value::from(init_rand());
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
    let upload = Value::from(init_rand());
    let download = Value::from(init_rand());
    let latency = Value::from(init_rand());
    let value = Value::Null;
    let aggregates: Vec<AggregateTmp> = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH input_data(id, created_at, updated_at, user_id, name, value) AS (
  VALUES
    (gen_random_uuid(), now(), now(), $1::uuid, 'Uptime',           $2::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'InteractiveExt',   $2::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Wootz',            $2::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Download',         $4::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Upload',           $3::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Latency',          $5::jsonb),
    (gen_random_uuid(), now(), now(), $1::uuid, 'Tasks',            $2::jsonb)
),
upsert AS (
  INSERT INTO aggregates (id, created_at, user_id, name, value, updated_at, dummy_updated_at)
  SELECT id, created_at, user_id, name, value, created_at, created_at
  FROM input_data
  ON CONFLICT (user_id, name) DO NOTHING
  RETURNING id, created_at, user_id, name, value, updated_at
)
SELECT id, created_at, user_id, name, value, updated_at FROM upsert
UNION ALL
SELECT a.id, a.created_at, a.user_id, a.name, a.value, a.updated_at FROM aggregates a
JOIN input_data i USING (user_id, name)
WHERE NOT EXISTS (
  SELECT 1
  FROM upsert u
  WHERE u.user_id = i.user_id AND u.name = i.name
);
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
