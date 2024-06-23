//use crate::domain::aggregate::{Aggregate, AggregateName};
//use chrono::Utc;
//use sqlx::{Postgres, Transaction};
//use uuid::Uuid;
//
//#[tracing::instrument(
//    name = "get_or_create_aggregate_by_user_and_name",
//    skip(transaction),
//    ret,
//    err
//)]
//pub(crate) async fn get_or_create_aggregate_by_user_and_name(
//    transaction: &mut Transaction<'_, Postgres>,
//    name: AggregateName,
//    user_id: Uuid,
//) -> anyhow::Result<Aggregate> {
//    let now = Utc::now();
//    let id = Uuid::new_v4();
//    let value = serde_json::Value::Null;
//    let aggregate = sqlx::query_as!(
//        Aggregate,
//        r#"
//        INSERT INTO aggregates
//        (id, created_at, user_id, name, value)
//        VALUES ($1, $2, $3, $4, $5)
//        ON CONFLICT (user_id, name)
//        DO UPDATE set user_id = $3
//        RETURNING id, created_at, user_id, name, value
//        "#,
//        id,
//        now,
//        user_id,
//        name.to_string(),
//        value
//    )
//    .fetch_one(&mut **transaction)
//    .await?;
//    Ok(aggregate)
//}
//
