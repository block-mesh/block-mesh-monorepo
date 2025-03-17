use crate::constants::INTRACT_USER_INFO_API_URL;
use crate::interfaces::server_api::{
    IntractIdentityType, IntractParams, IntractResp, IntractRespData,
};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

pub async fn get_intract_user_details(
    identity: &str,
    identity_type: &IntractIdentityType,
) -> anyhow::Result<IntractRespData> {
    let client = reqwest::Client::new();
    let response: Value = client
        .get(INTRACT_USER_INFO_API_URL)
        .header(
            "Authorization",
            &format!("Bearer {}", env::var("INTRACT_API_KEY")?),
        )
        .query(&IntractParams {
            identity: identity.to_string(),
            identityType: identity_type.clone(),
        })
        .send()
        .await?
        .json()
        .await?;
    let response: IntractResp = serde_json::from_value(response)?;
    Ok(response.data)
}

pub fn calc_bonus(data: Value) -> anyhow::Result<f64> {
    let mut score = 0.0;
    let intract_data: HashMap<String, Value> = serde_json::from_value(data)?;
    for (key, value) in intract_data.iter() {
        if key == "evmAddress" {
            score += 1_000.0;
        } else if key == "twitterId" {
            score += 1_000.0;
        } else if key == "discordId" {
            score += 1_000.0;
        } else if key == "solAddress" {
            score += 1_000.0;
        } else if key == "telegramId" {
            score += 1_000.0;
        } else if key == "email" {
            score += 1_000.0;
        } else if key == "kyc" && value.is_array() && value.as_array().unwrap().len() > 0 {
            score += value.as_array().unwrap().len() as f64 * 1_000.0;
        } else if key == "xp" {
            score += value.as_f64().unwrap_or(0.0);
        }
    }
    Ok(score)
}
