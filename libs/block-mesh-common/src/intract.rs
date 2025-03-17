use crate::constants::INTRACT_USER_INFO_API_URL;
use crate::interfaces::server_api::{
    IntractIdentityType, IntractParams, IntractResp, IntractRespData,
};
use serde_json::Value;
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
