use crate::errors::error::Error;
use crate::startup::application::AppState;
use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::GetTwitterProfileDetails;
use block_mesh_common::reqwest::http_client;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterApiProfileResponse {
    pub created_at: NaiveDate,
    pub screen_name: String,
}

pub fn get_key_deep(key: &str, json: &Value) -> Option<Value> {
    if !json.is_object() {
        return None;
    }
    if let Some(v) = json.get(&key) {
        return Some(v.clone());
    }
    let j = json.as_object().unwrap();
    for (_k, v) in j.iter() {
        let res = get_key_deep(key, v);
        if let Some(value) = res {
            return Some(value);
        }
    }
    None
}

pub async fn get_twitter_profile(username: &str) -> anyhow::Result<TwitterApiProfileResponse> {
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Params {
        screenName: String,
        apiKey: String,
        resFormat: String,
    }
    let client = http_client(DeviceType::AppServer);
    let value: Value = client
        .get(env::var("TWITTER_PROFILE_API_URL").expect("could not find TWITTER_API_URL"))
        .query(&Params {
            screenName: username.to_string(),
            apiKey: env::var("TWITTER_API_TOKEN").expect("could not find TWITTER_API_TOKEN"),
            resFormat: "json".to_string(),
        })
        .header(
            "x-rapidapi-host",
            env::var("TWITTER_API_HOST").expect("could not find TWITTER_API_HOST"),
        )
        .header(
            "x-rapidapi-key",
            env::var("TWITTER_API_TOKEN_TOKEN").expect("could not find TWITTER_API_TOKEN_TOKEN"),
        )
        .send()
        .await?
        .json()
        .await?;
    let value: Value = serde_json::from_value(value)?;
    let screen_name = get_key_deep("screen_name", &value)
        .ok_or(anyhow!("Cannot find screen_name"))?
        .to_string();
    let created_at = get_key_deep("created_at", &value)
        .ok_or(anyhow!("Cannot find created_at"))?
        .to_string();
    let created_at = created_at.replace('"', "");
    let format = "%a %b %d %H:%M:%S %z %Y";
    let created_at: DateTime<FixedOffset> = DateTime::parse_from_str(&created_at, format)?;
    let created_at: NaiveDate = created_at.date_naive();
    Ok(TwitterApiProfileResponse {
        screen_name,
        created_at,
    })
}

#[tracing::instrument(name = "get_twitter_profile_details", skip_all)]
pub async fn handler(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetTwitterProfileDetails>,
) -> Result<impl IntoResponse, Error> {
    if query.code.is_empty() || query.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        return Err(Error::Anyhow(anyhow!("Bad admin param")));
    }
    let results = get_twitter_profile(&query.username).await?;
    Ok(Json(results))
}
