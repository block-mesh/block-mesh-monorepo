use crate::constants::BLOCK_MESH_FEATURE_FLAGS;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FLAGS: [&str; 11] = [
    "enrich_ip_and_cleanup_in_background",
    "submit_bandwidth_run_background",
    "send_cleanup_to_rayon",
    "polling_interval",
    "tx_analytics_agg",
    "touch_users_ip",
    "submit_bandwidth_via_channel",
    "report_uptime_daily_stats_via_channel",
    "send_to_worker",
    "server_side_ws",
    "use_websocket",
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

#[tracing::instrument(name = "get_all_flags", skip_all, ret, err)]
pub async fn get_all_flags(client: &Client) -> anyhow::Result<HashMap<String, FlagValue>> {
    let mut flags: HashMap<String, FlagValue> = HashMap::new();
    for flag in FLAGS {
        tracing::info!("Fetching flag {:?}", flag);
        let value = get_flag_value(flag, client).await?.unwrap();
        tracing::info!("Fetching flag {:?} from http , value = {:?}", flag, value);
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
        tracing::info!("Finished fetching flag {:?} , value = {:?}", flag, value);
    }
    Ok(flags)
}

#[tracing::instrument(name = "get_flag_value", skip_all, ret, err)]
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
    use tracing_test::traced_test;
    use uuid::Uuid;

    pub fn get_client() -> Client {
        ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .cookie_store(true)
            .user_agent("curl/8.7.1")
            .build()
            .unwrap_or_default()
    }

    #[tokio::test]
    #[traced_test]
    async fn test_test_boolean_false() {
        let client = get_client();
        let value = get_flag_value("test_boolean_false", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(false, value);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_test_boolean_true() {
        let client = get_client();
        let value = get_flag_value("test_boolean_true", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(true, value);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_missing_value() {
        let client = get_client();
        let uuid = Uuid::new_v4();
        let value = get_flag_value(&uuid.to_string(), &client).await;
        assert!(value.is_err());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_polling_value() {
        let client = get_client();
        let value = get_flag_value("polling_interval", &client).await;
        assert!(value.is_ok());
        let value = value.unwrap();
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_number());
        let _ = FlagValue::Number(value.as_f64().unwrap());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_all_values() {
        let client = get_client();
        let _values = get_all_flags(&client).await.unwrap();
    }
}
