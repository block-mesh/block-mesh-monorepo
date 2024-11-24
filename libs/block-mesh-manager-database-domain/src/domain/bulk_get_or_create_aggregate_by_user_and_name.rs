use crate::domain::aggregate::{Aggregate, AggregateTmp};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "bulk_get_or_create_aggregate_by_user_and_name", skip_all)]
pub async fn bulk_get_or_create_aggregate_by_user_and_name(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<Aggregate>> {
    let value = serde_json::Value::Null;
    let aggregates: Vec<AggregateTmp> = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH input_data AS (
    -- Input data
    SELECT *
    FROM (
    VALUES
        (gen_random_uuid(), now(), now(), $1::uuid, 'Uptime', $2::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Download', $2::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Upload', $2::jsonb),
        (gen_random_uuid(), now(), now(), $1::uuid, 'Latency', $2::jsonb),
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
SELECT id, created_at, user_id, name, value, updated_at
FROM inserted
UNION ALL
SELECT id, created_at, user_id, name, value, updated_at
FROM extant;
"#,
        user_id,
        value
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
