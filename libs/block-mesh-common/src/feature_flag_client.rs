use crate::constants::BLOCK_MESH_FEATURE_FLAGS;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

const FLAGS: [&str; 3] = [
    "enrich_ip_and_cleanup_in_background",
    "submit_bandwidth_run_background",
    "send_cleanup_to_rayon",
];

pub async fn get_all_flags(client: &Client) -> anyhow::Result<HashMap<String, bool>> {
    let mut flags: HashMap<String, bool> = HashMap::new();
    for flag in FLAGS {
        let value = get_flag_value(flag, client).await?.unwrap();
        flags.insert(flag.to_string(), value.as_bool().unwrap());
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
}
