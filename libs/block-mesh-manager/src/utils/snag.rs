use anyhow::anyhow;
use reqwest::Client;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SnagConfig {
    pub base_url: String,
    pub api_key: String,
    pub external_rule_extension: String,
    pub external_rule_wallet: String,
    pub external_rule_mobile: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagUserMetadataRequest {
    id: String,
    external_identifier: String,
    wallet_address: String,
    email_address: String,
    display_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagRuleCompleteRequest {
    wallet_address: String,
}

fn generated_wallet_address() -> String {
    Keypair::new().pubkey().to_string()
}

async fn send_create_or_update_user(
    client: &Client,
    config: &SnagConfig,
    body: &SnagUserMetadataRequest,
) -> anyhow::Result<()> {
    let request = client
        .post(format!("{}/api/users/metadatas", config.base_url))
        .header("X-API-KEY", &config.api_key)
        .header("Content-Type", "application/json")
        .json(body);

    let response = request.send().await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if status.is_success() || status == reqwest::StatusCode::CONFLICT {
        if status == reqwest::StatusCode::CONFLICT {
            tracing::warn!("Snag user upsert returned conflict: {}", response_body);
        }
        return Ok(());
    }
    Err(anyhow!(
        "Snag user upsert failed with status {}: {}",
        status,
        response_body
    ))
}

async fn send_complete_rule(
    client: &Client,
    config: &SnagConfig,
    wallet_address: &str,
) -> anyhow::Result<()> {
    let request = client
        .post(format!(
            "{}/api/loyalty/rules/{}/complete",
            config.base_url, config.external_rule_extension
        ))
        .header("X-API-KEY", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&SnagRuleCompleteRequest {
            wallet_address: wallet_address.to_string(),
        });
    let response = request.send().await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if status.is_success() {
        return Ok(());
    }
    Err(anyhow!(
        "Snag loyalty completion failed with status {}: {}",
        status,
        response_body
    ))
}

#[tracing::instrument(name = "snag_sync_first_activation", skip(client, config))]
pub async fn sync_first_activation(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: Option<String>,
) -> anyhow::Result<()> {
    let wallet_address = wallet_address.unwrap_or_else(generated_wallet_address);
    let body = SnagUserMetadataRequest {
        id: user_id.to_string(),
        external_identifier: user_id.to_string(),
        wallet_address: wallet_address.clone(),
        email_address: email.clone(),
        display_name: email,
    };

    send_create_or_update_user(&client, &config, &body).await?;
    send_complete_rule(&client, &config, &wallet_address).await?;
    Ok(())
}

#[tracing::instrument(name = "snag_sync_wallet_update", skip(client, config, wallet_address))]
pub async fn sync_wallet_update(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: String,
) -> anyhow::Result<()> {
    let body = SnagUserMetadataRequest {
        id: user_id.to_string(),
        external_identifier: user_id.to_string(),
        wallet_address,
        email_address: email.clone(),
        display_name: email,
    };

    send_create_or_update_user(&client, &config, &body).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_user_metadata_payload_in_camel_case() {
        let payload = SnagUserMetadataRequest {
            id: "user-id".to_string(),
            external_identifier: "user-id".to_string(),
            wallet_address: "wallet".to_string(),
            email_address: "user@example.com".to_string(),
            display_name: "user@example.com".to_string(),
        };

        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["id"], "user-id");
        assert_eq!(json["externalIdentifier"], "user-id");
        assert_eq!(json["walletAddress"], "wallet");
        assert_eq!(json["emailAddress"], "user@example.com");
        assert_eq!(json["displayName"], "user@example.com");
    }

    #[test]
    fn generated_wallet_address_looks_like_a_solana_pubkey() {
        let wallet = generated_wallet_address();
        assert!(!wallet.is_empty());
        assert!(wallet.len() >= 32);
    }
}
