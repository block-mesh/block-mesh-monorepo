use crate::constants::INTRACT_USER_INFO_API_URL;
use crate::interfaces::server_api::{
    IntractError, IntractIdentityType, IntractParams, IntractResp, IntractResponses,
};
use anyhow::anyhow;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

pub async fn get_intract_user_details(
    identity: &str,
    identity_type: &IntractIdentityType,
) -> anyhow::Result<IntractResponses> {
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
    if let Ok(resp) = serde_json::from_value::<IntractResp>(response.clone()) {
        return Ok(IntractResponses::IntractRespData(resp.data));
    }
    if let Ok(resp) = serde_json::from_value::<IntractError>(response) {
        return Ok(IntractResponses::IntractError(resp));
    }
    Err(anyhow!("Intract failure"))
}

pub fn calc_bonus(data: Value) -> anyhow::Result<f64> {
    let mut score = 0.0;
    let intract_data: HashMap<String, Value> = serde_json::from_value(data)?;
    for (key, value) in intract_data.iter() {
        if key == "evmAddress"
            || key == "twitterId"
            || key == "discordId"
            || key == "solAddress"
            || key == "telegramId"
            || key == "email"
        {
            score += 1_000.0;
        } else if key == "kyc" && value.is_array() && !value.as_array().unwrap().is_empty() {
            score += value.as_array().unwrap().len() as f64 * 1_000.0;
        } else if key == "pohMintStatus" && value.as_bool().unwrap_or_default() {
            score += 2_000.0;
        } else if key == "xp" {
            score += value.as_f64().unwrap_or(0.0);
        }
    }
    Ok(score)
}
