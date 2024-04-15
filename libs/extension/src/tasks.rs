use crate::utils::log::log;
use anyhow::anyhow;
use block_mesh_common::interface::{
    GetTaskRequest, GetTaskResponse, SubmitTaskRequest, SubmitTaskResponse,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunTaskResponse {
    pub status: i32,
    pub raw: String,
}

pub async fn get_task(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
) -> anyhow::Result<GetTaskResponse> {
    let body: GetTaskRequest = GetTaskRequest {
        email: email.to_string(),
        api_token: *api_token,
    };
    let response: GetTaskResponse = reqwest::Client::new()
        .post(format!("{}/api/get_task", base_url))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

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
        _ => {
            log!("Unsupported method");
            return Err(anyhow!("Unsupported method"));
        }
    };

    if headers.is_some() {
        let mut headers_map = HeaderMap::new();
        let headers = headers.clone().unwrap();
        headers.as_object().unwrap().into_iter().for_each(|(k, v)| {
            let header_name = HeaderName::from_str(k).unwrap();
            let header_value = HeaderValue::from_str(v.as_str().unwrap()).unwrap();
            headers_map.insert(header_name, header_value);
        });
        client = client.headers(headers_map)
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
            log!("{e}");
            Err(anyhow!("{e}"))
        }
    }
}

pub async fn submit_task(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
    task_id: &Uuid,
    response_code: Option<i32>,
    response_raw: Option<String>,
) -> anyhow::Result<SubmitTaskResponse> {
    let body: SubmitTaskRequest = SubmitTaskRequest {
        email: email.to_string(),
        api_token: *api_token,
        task_id: *task_id,
        response_code,
        response_raw,
    };
    let response: SubmitTaskResponse = reqwest::Client::new()
        .post(format!("{}/api/submit_task", base_url))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
