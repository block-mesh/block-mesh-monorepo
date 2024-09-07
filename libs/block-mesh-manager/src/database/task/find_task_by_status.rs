use crate::domain::task::TaskMethod;
use crate::domain::task::TaskStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTask {
    pub id: Uuid,
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
}

#[tracing::instrument(
    name = "Find task status",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn find_task_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    status: TaskStatus,
) -> anyhow::Result<Option<GetTask>> {
    let task = sqlx::query_as!(
        GetTask,
        r#"
        SELECT
        id,
        url,
        method,
        headers,
        body
        FROM tasks
        WHERE status = $1
        LIMIT 1
        "#,
        status.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(task)
}
