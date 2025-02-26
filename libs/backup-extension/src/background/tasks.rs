use anyhow::anyhow;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{
    GetTaskRequest, GetTaskResponse, RunTaskResponse, SubmitTaskRequest, SubmitTaskResponse,
};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use leptos_dom::tracing;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use speed_test::Metadata;
use std::str::FromStr;
use uuid::Uuid;

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
        .post(format!(
            "{}/{}/api{}",
            base_url,
            DeviceType::Extension,
            RoutesEnum::Api_GetToken
        ))
        .query(&body)
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

    if let Some(headers) = headers {
        let mut headers_map = HeaderMap::new();
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
        response_body: None,
    };
    let response = reqwest::Client::new()
        .post(format!(
            "{}/{}/api{}",
            base_url,
            DeviceType::Extension,
            RoutesEnum::Api_SubmitTask
        ))
        .query(&query)
        .body(response_raw)
        .send()
        .await?;
    let response: SubmitTaskResponse = response.json().await?;
    Ok(response)
}
