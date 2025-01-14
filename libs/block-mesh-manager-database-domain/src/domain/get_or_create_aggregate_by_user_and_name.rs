use crate::domain::aggregate::{Aggregate, AggregateName, AggregateTmp};
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

// #[tracing::instrument(name = "get_or_create_aggregate_by_user_and_name", skip_all)]
// pub async fn get_or_create_aggregate_by_user_and_name(
//     transaction: &mut Transaction<'_, Postgres>,
//     name: AggregateName,
//     user_id: &Uuid,
// ) -> anyhow::Result<Aggregate> {
//     let now = Utc::now();
//     let id = Uuid::new_v4();
//     let value = serde_json::Value::Null;
//     let aggregate = sqlx::query_as!(
//         Aggregate,
//         r#"
//         INSERT
//         INTO aggregates (id, created_at, user_id, name, value, updated_at)
//         VALUES ($1, $2, $3, $4, $5, $6)
//         ON CONFLICT (user_id, name) DO UPDATE SET dummy_updated_at = $6
//         RETURNING id, created_at, user_id, name, value, updated_at
//         "#,
//         id,
//         now.clone(),
//         user_id,
//         name.to_string(),
//         value,
//         now
//     )
//     .fetch_one(&mut **transaction)
//     .await?;
//     Ok(aggregate)
// }

#[tracing::instrument(name = "get_or_create_aggregate_by_user_and_name", skip_all)]
pub async fn get_or_create_aggregate_by_user_and_name(
    transaction: &mut Transaction<'_, Postgres>,
    name: AggregateName,
    user_id: &Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH extant AS (
	SELECT id,created_at,user_id,name,value,updated_at
	FROM aggregates
	WHERE (user_id, name) = ($3,$4)
),
inserted AS (
INSERT INTO aggregates (id,	created_at,	user_id, name, value, updated_at, dummy_updated_at)
SELECT $1,$2,$3,$4,$5,$2,$2
WHERE
	NOT EXISTS (SELECT	FROM extant)
	RETURNING id, created_at,user_id, name, value, updated_at
)
SELECT id,created_at,user_id,name,value,updated_at
FROM inserted
UNION ALL
SELECT id,created_at,user_id,name,value,updated_at
FROM extant;
"#,
        id,
        now.clone(),
        user_id,
        name.to_string(),
        value
    )
    .fetch_one(&mut **transaction)
    .await?;
    let aggregate = Aggregate {
        id: aggregate.id.expect("MISSING ID"),
        user_id: aggregate.user_id.expect("MISSING USER ID"),
        name: aggregate.name.expect("MISSING NAME").into(),
        value: aggregate.value.expect("MISSING VALUE"),
        created_at: aggregate.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
        updated_at: aggregate.updated_at.expect("MISSING TIMESTAMP UPDATED_AT"),
    };
    Ok(aggregate)
}

#[tracing::instrument(name = "get_or_create_aggregate_by_user_and_name", skip_all)]
pub async fn get_or_create_aggregate_by_user_and_name_str(
    transaction: &mut Transaction<'_, Postgres>,
    name: &str,
    user_id: &Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        AggregateTmp,
        r#"
WITH extant AS (
	SELECT id,created_at,user_id,name,value,updated_at
	FROM aggregates
	WHERE (user_id, name) = ($3,$4)
),
inserted AS (
INSERT INTO aggregates (id,	created_at,	user_id, name, value, updated_at, dummy_updated_at)
SELECT $1,$2,$3,$4,$5,$2,$2
WHERE
	NOT EXISTS (SELECT	FROM extant)
	RETURNING id, created_at,user_id, name, value, updated_at
)
SELECT id,created_at,user_id,name,value,updated_at
FROM inserted
UNION ALL
SELECT id,created_at,user_id,name,value,updated_at
FROM extant;
"#,
        id,
        now.clone(),
        user_id,
        name.to_string(),
        value
    )
    .fetch_one(&mut **transaction)
    .await?;
    let aggregate = Aggregate {
        id: aggregate.id.expect("MISSING ID"),
        user_id: aggregate.user_id.expect("MISSING USER ID"),
        name: aggregate.name.expect("MISSING NAME").into(),
        value: aggregate.value.expect("MISSING VALUE"),
        created_at: aggregate.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
        updated_at: aggregate.updated_at.expect("MISSING TIMESTAMP UPDATED_AT"),
    };
    Ok(aggregate)
}
