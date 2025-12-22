use block_mesh_manager_database_domain::domain::task::TaskStatus;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use time::{Duration, OffsetDateTime};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct TmpRpcResults {
    pub url: Option<String>,
    pub country: String,
    pub response_code: Option<i32>,
    pub latency: Option<f64>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct RpcResults {
    pub url: String,
    pub country: String,
    pub response_code: i32,
    pub latency: f64,
    pub count: i64,
    pub provider: String,
}

pub async fn get_tasks_rpc_results(
    transaction: &mut Transaction<'_, Postgres>,
    duration: u64,
) -> anyhow::Result<Vec<RpcResults>> {
    let now = OffsetDateTime::now_utc();
    let duration = now - Duration::seconds(duration as i64);
    let status = [TaskStatus::Failed, TaskStatus::Completed]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let rpc_results = sqlx::query_as!(
        TmpRpcResults,
        r#"
        SELECT
        	regexp_replace(url, '\?.*$', '') as url,
        	country,
        	response_code,
        	AVG(response_time) as latency,
        	COALESCE(COUNT(*), 0) as count
        FROM tasks
        WHERE
            created_at > $1
        AND
        	status = ANY($2::text[])
        GROUP BY url, country, response_code
        "#,
        duration,
        &status
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(rpc_results
        .into_iter()
        .filter_map(|i| {
            let rpc_results = RpcResults {
                url: i.url.clone().unwrap_or_default(),
                country: i.country,
                response_code: i.response_code.unwrap_or_default(),
                latency: i.latency.unwrap_or_default(),
                count: i.count.unwrap_or_default(),
                provider: "N/A".to_string(), // RpcName::from_url(&i.url.clone().unwrap_or_default()).to_string(),
            };
            if rpc_results.provider == *"Invalid" // RpcName::Invalid.to_string()
                || rpc_results.provider.is_empty()
            {
                None
            } else {
                Some(rpc_results)
            }
        })
        .collect())
}
