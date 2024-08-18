use crate::constants::BLOCK_MESH_FEATURE_FLAGS;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FLAGS: [&str; 4] = [
    "enrich_ip_and_cleanup_in_background",
    "submit_bandwidth_run_background",
    "send_cleanup_to_rayon",
    "polling_interval",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FlagValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl TryInto<bool> for FlagValue {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            FlagValue::Boolean(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl TryInto<f64> for FlagValue {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            FlagValue::Number(n) => Ok(n),
            _ => Err(()),
        }
    }
}

pub async fn get_all_flags(client: &Client) -> anyhow::Result<HashMap<String, FlagValue>> {
    let mut flags: HashMap<String, FlagValue> = HashMap::new();
    for flag in FLAGS {
        let value = get_flag_value(flag, client).await?.unwrap();
        if value.is_boolean() {
            flags.insert(
                flag.to_string(),
                FlagValue::Boolean(value.as_bool().unwrap()),
            );
        } else if value.is_string() {
            flags.insert(flag.to_string(), FlagValue::String(value.to_string()));
        } else if value.is_number() {
            flags.insert(flag.to_string(), FlagValue::Number(value.as_f64().unwrap()));
        }
    }
    Ok(flags)
}

pub async fn get_flag_value(flag: &str, client: &Client) -> anyhow::Result<Option<Value>> {
    let url = format!("{}/read-flag/{}", BLOCK_MESH_FEATURE_FLAGS, flag);
    let response: Value = client.get(&url).send().await?.json().await?;
    Ok(Some(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::ClientBuilder;
    use std::time::Duration;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_test_boolean_false() {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let value = get_flag_value("test_boolean_false", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(false, value);
    }

    #[tokio::test]
    async fn test_test_boolean_true() {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let value = get_flag_value("test_boolean_true", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(true, value);
    }

    #[tokio::test]
    async fn test_missing_value() {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let uuid = Uuid::new_v4();
        let value = get_flag_value(&uuid.to_string(), &client).await;
        assert!(value.is_err());
    }

    #[tokio::test]
    async fn test_polling_value() {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let value = get_flag_value("polling_interval", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_number());
        let _ = FlagValue::Number(value.as_f64().unwrap());
    }

    #[tokio::test]
    async fn test_all_values() {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let _values = get_all_flags(&client).await.unwrap();
    }
}
