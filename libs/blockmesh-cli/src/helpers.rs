use anyhow::anyhow;
use block_mesh_common::constants::BLOCK_MESH_APP_SERVER;
use block_mesh_common::interfaces::server_api::{
    GetTaskRequest, GetTaskResponse, ReportBandwidthRequest, ReportBandwidthResponse,
    ReportUptimeRequest, ReportUptimeResponse, RunTaskResponse, SubmitTaskRequest,
    SubmitTaskResponse,
};
use block_mesh_common::interfaces::server_api::{GetTokenResponse, LoginForm};
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use speed_test::download::test_download;
use speed_test::latency::test_latency;
use speed_test::metadata::fetch_metadata;
use speed_test::upload::test_upload;
use speed_test::Metadata;
use std::cmp;
use std::str::FromStr;
use uuid::Uuid;

pub async fn login(login_form: LoginForm) -> anyhow::Result<Uuid> {
    let url = format!("{}/api/get_token", BLOCK_MESH_APP_SERVER);
    let client = reqwest::Client::new();
    let response: GetTokenResponse = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&login_form)
        .send()
        .await
        .map_err(|e| anyhow!(e.to_string()))?
        .json()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    match response.api_token {
        Some(api_token) => Ok(api_token),
        None => Err(anyhow!("missing api_token")),
    }
}

#[tracing::instrument(name = "report_uptime", skip(api_token), err)]
pub async fn report_uptime(email: &str, api_token: &str) -> anyhow::Result<()> {
    let api_token = Uuid::from_str(api_token).map_err(|_| anyhow!("Invalid UUID"))?;
    let metadata = fetch_metadata().await.unwrap_or_default();

    let query = ReportUptimeRequest {
        email: email.to_string(),
        api_token,
        ip: if metadata.ip.is_empty() {
            None
        } else {
            Some(metadata.ip)
        },
    };

    if let Ok(response) = reqwest::Client::new()
        .post(format!("{}/api/report_uptime", BLOCK_MESH_APP_SERVER))
        .query(&query)
        .send()
        .await
    {
        let _ = response.json::<ReportUptimeResponse>().await;
    }
    Ok(())
}

#[tracing::instrument(name = "get_task", level = "trace", skip(api_token), err)]
pub async fn get_task(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
) -> anyhow::Result<Option<GetTaskResponse>> {
    let body: GetTaskRequest = GetTaskRequest {
        email: email.to_string(),
        api_token: *api_token,
    };

    let response: Option<GetTaskResponse> = reqwest::Client::new()
        .post(format!("{}/api/get_task", base_url))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[tracing::instrument(name = "run_task", err)]
pub async fn run_task(
    url: &str,
    method: &str,
    headers: Option<Value>,
    body: Option<Value>,
) -> anyhow::Result<RunTaskResponse> {
    let client = reqwest::Client::new();
    let mut client = match method {
        "GET" => client.get(url),
        "POST" => match body {
            Some(v) => client.post(url).json(&v),
            None => client.post(url),
        },
        method => {
            tracing::error!("Unsupported method: {}", method);
            return Err(anyhow!("Unsupported method: {}", method));
        }
    };

    if headers.is_some() {
        let mut headers_map = HeaderMap::new();
        let headers = headers.unwrap();
        if headers.is_object() {
            headers.as_object().unwrap().into_iter().for_each(|(k, v)| {
                let header_name = HeaderName::from_str(k).unwrap();
                let header_value = HeaderValue::from_str(v.as_str().unwrap()).unwrap();
                headers_map.insert(header_name, header_value);
            });
            client = client.headers(headers_map)
        }
    }

    let response = client.send().await;
    match response {
        Ok(v) => {
            let status = v.status().as_u16();
            let raw = v.text().await?;

            Ok(RunTaskResponse {
                status: status.into(),
                raw,
            })
        }
        Err(e) => {
            tracing::error!("run_task error: {e}");
            Err(anyhow!("run_task error: {e}"))
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(name = "submit_task", skip(api_token, response_raw), err)]
pub async fn submit_task(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
    task_id: &Uuid,
    response_code: i32,
    response_raw: String,
    metadata: &Metadata,
    response_time: f64,
) -> anyhow::Result<SubmitTaskResponse> {
    let query: SubmitTaskRequest = SubmitTaskRequest {
        email: email.to_string(),
        api_token: *api_token,
        task_id: *task_id,
        response_code: Some(response_code),
        country: Option::from(metadata.country.clone()),
        ip: Option::from(metadata.ip.clone()),
        asn: Option::from(metadata.asn.clone()),
        colo: Option::from(metadata.colo.clone()),
        response_time: Option::from(response_time),
    };
    let response = reqwest::Client::new()
        .post(format!("{}/api/submit_task", base_url))
        .query(&query)
        .body(response_raw)
        .send()
        .await?;
    let response: SubmitTaskResponse = response.json().await?;
    Ok(response)
}

pub async fn task_poller(email: &str, api_token: &str) -> anyhow::Result<()> {
    let api_token = Uuid::from_str(api_token).map_err(|_| anyhow!("Invalid UUID"))?;
    let task = match get_task(BLOCK_MESH_APP_SERVER, email, &api_token).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("get_task error: {e}");
            return Err(e);
        }
    };
    let metadata = fetch_metadata().await.unwrap_or_default();
    let task = match task {
        Some(v) => v,
        None => {
            return Err(anyhow!("Task not found"));
        }
    };
    let start = Utc::now();

    let finished_task = match run_task(&task.url, &task.method, task.headers, task.body).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("finished_task: error: {e}");
            let end = Utc::now();
            let response_time = cmp::max((end - start).num_milliseconds(), 1) as f64;
            match submit_task(
                BLOCK_MESH_APP_SERVER,
                email,
                &api_token,
                &task.id,
                520,
                "".to_string(),
                &metadata,
                response_time,
            )
            .await
            {
                Ok(_) => {
                    tracing::info!("successfully submitted failed task");
                }
                Err(e) => {
                    tracing::error!("submit_task: error: {e}");
                }
            }
            return Err(anyhow!("submit_task errored"));
        }
    };
    let end = Utc::now();
    let response_time = cmp::max((end - start).num_milliseconds(), 1) as f64;

    match submit_task(
        BLOCK_MESH_APP_SERVER,
        email,
        &api_token,
        &task.id,
        finished_task.status,
        finished_task.raw,
        &metadata,
        response_time,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("successfully submitted task");
        }
        Err(e) => {
            tracing::error!("submit_task: error: {e}");
        }
    };
    Ok(())
}

#[tracing::instrument(name = "submit_bandwidth", err)]
pub async fn submit_bandwidth(
    email: &str,
    api_token: &str,
) -> anyhow::Result<ReportBandwidthResponse> {
    let api_token = Uuid::from_str(api_token).map_err(|_| anyhow!("Invalid UUID"))?;
    let download_speed = test_download(100_000).await.unwrap_or_default();
    let upload_speed = test_upload(100_000).await.unwrap_or_default();
    let latency = test_latency().await.unwrap_or_default();
    let metadata = fetch_metadata().await.unwrap_or_default();

    let body = ReportBandwidthRequest {
        email: email.to_string(),
        api_token,
        download_speed,
        upload_speed,
        latency,
        city: metadata.city,
        country: metadata.country,
        ip: metadata.ip,
        asn: metadata.asn,
        colo: metadata.colo,
    };

    let response = reqwest::Client::new()
        .post(format!("{}/api/submit_bandwidth", BLOCK_MESH_APP_SERVER))
        .json(&body)
        .send()
        .await?;
    let response: ReportBandwidthResponse = response.json().await?;
    Ok(response)
}
