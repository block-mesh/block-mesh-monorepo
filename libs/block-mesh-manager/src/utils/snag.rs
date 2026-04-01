use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Keypair, Signer};
use std::env;
use time::{Date, Month, OffsetDateTime, Time};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SnagConfig {
    pub base_url: String,
    pub api_key: String,
    pub external_rule_email_registered: String,
    pub external_rule_extension: String,
    pub external_rule_wallet: String,
    pub external_rule_mobile: String,
    pub website_id: String,
    pub organization_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnagWalletVerification {
    message: Option<String>,
    signature: Option<String>,
    verified_locally: bool,
}

impl SnagWalletVerification {
    pub fn verified_solana(message: String, signature: String) -> Self {
        Self {
            message: Some(message),
            signature: Some(signature),
            verified_locally: true,
        }
    }

    pub fn verified_locally_only() -> Self {
        Self {
            message: None,
            signature: None,
            verified_locally: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SnagEmailRewardOutcome {
    Pending,
    Consumed,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SnagFirstActivationOutcome {
    Pending,
    Consumed,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagUserMetadataRequest {
    id: String,
    external_identifier: String,
    wallet_address: String,
    email_address: String,
    display_name: String,
    user_group_external_identifier: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagRuleCompleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagConnectUserRequest {
    website_id: String,
    organization_id: String,
    wallet_type: &'static str,
    wallet_address: String,
    verification_data: SnagConnectVerificationRequest,
    user_id: Uuid,
    confirm_disconnect: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SnagConnectVerificationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    verified_locally: bool,
}

#[derive(Deserialize)]
struct SnagUserMetadataResponse {
    success: Option<bool>,
    message: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SnagUsersResponse {
    #[serde(default)]
    data: Vec<SnagUserLookup>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SnagUserLookup {
    id: Uuid,
    #[serde(default)]
    wallet_address: Option<String>,
    #[serde(default)]
    email_address: Option<String>,
    #[serde(default)]
    email_verified_at: Option<String>,
    #[serde(default)]
    user_metadata: Vec<SnagUserMetadataLookup>,
}

#[derive(Deserialize)]
struct SnagRuleCompleteResponse {
    message: Option<String>,
    rewarded: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SnagUserMetadataLookup {
    #[serde(default)]
    email_address: Option<String>,
    #[serde(default)]
    email_verified_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedSnagUser {
    id: Uuid,
    wallet_address: Option<String>,
    email_verified: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SnagRuleKind {
    EmailRegistered,
    Extension,
    Wallet,
}

impl SnagRuleKind {
    fn external_rule_id(self, config: &SnagConfig) -> &str {
        match self {
            Self::EmailRegistered => &config.external_rule_email_registered,
            Self::Extension => &config.external_rule_extension,
            Self::Wallet => &config.external_rule_wallet,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SnagRuleTarget {
    Wallet(String),
    User(Uuid),
}

fn generated_wallet_address() -> String {
    Keypair::new().pubkey().to_string()
}

const SNAG_CUTOFF_DATE_ENV: &str = "SNAG_CUTOFF_DATE";
const DEFAULT_SNAG_CUTOFF_DATE: &str = "2026-03-24";

fn build_cutoff_date(year: i32, month: Month, day: u8) -> Option<OffsetDateTime> {
    Date::from_calendar_date(year, month, day)
        .ok()
        .map(|date| date.with_time(Time::MIDNIGHT).assume_utc())
}

fn default_snag_cutoff_date() -> OffsetDateTime {
    build_cutoff_date(2026, Month::March, 24).expect("valid default Snag cutoff date")
}

fn parse_snag_cutoff_date(raw: &str) -> Option<OffsetDateTime> {
    let mut parts = raw.trim().split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = Month::try_from(parts.next()?.parse::<u8>().ok()?).ok()?;
    let day = parts.next()?.parse::<u8>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    build_cutoff_date(year, month, day)
}

pub fn snag_cutoff_date() -> OffsetDateTime {
    match env::var(SNAG_CUTOFF_DATE_ENV) {
        Ok(raw) => match parse_snag_cutoff_date(&raw) {
            Some(date) => date,
            None => {
                tracing::warn!(
                    "invalid {} value {:?}, expected YYYY-MM-DD; falling back to {}",
                    SNAG_CUTOFF_DATE_ENV,
                    raw,
                    DEFAULT_SNAG_CUTOFF_DATE
                );
                default_snag_cutoff_date()
            }
        },
        Err(_) => default_snag_cutoff_date(),
    }
}

#[cfg(test)]
fn resolve_snag_cutoff_date(raw: Option<&str>) -> OffsetDateTime {
    raw.and_then(parse_snag_cutoff_date)
        .unwrap_or_else(default_snag_cutoff_date)
}

#[cfg(test)]
fn utc_midnight(year: i32, month: Month, day: u8) -> OffsetDateTime {
    build_cutoff_date(year, month, day).expect("valid test date")
}

pub fn is_snag_eligible_user(created_at: OffsetDateTime) -> bool {
    created_at >= snag_cutoff_date()
}

fn external_identifier(user_id: Uuid) -> String {
    user_id.to_string()
}

fn user_group_external_identifier(user_id: Uuid) -> String {
    format!("bm-user-group:{user_id}")
}

fn build_user_metadata_request(
    user_id: Uuid,
    email: String,
    wallet_address: String,
) -> SnagUserMetadataRequest {
    SnagUserMetadataRequest {
        id: user_id.to_string(),
        external_identifier: external_identifier(user_id),
        wallet_address,
        email_address: email.clone(),
        display_name: email,
        user_group_external_identifier: user_group_external_identifier(user_id),
    }
}

fn build_connect_user_request(
    config: &SnagConfig,
    user_id: Uuid,
    wallet_address: String,
    verification: SnagWalletVerification,
) -> SnagConnectUserRequest {
    SnagConnectUserRequest {
        website_id: config.website_id.clone(),
        organization_id: config.organization_id.clone(),
        wallet_type: "solana",
        wallet_address,
        verification_data: SnagConnectVerificationRequest {
            message: verification.message,
            signature: verification.signature,
            verified_locally: verification.verified_locally,
        },
        user_id,
        confirm_disconnect: true,
    }
}

impl SnagUserLookup {
    fn email_verified(&self) -> bool {
        self.email_verified_at.is_some()
            || self
                .user_metadata
                .iter()
                .any(|metadata| metadata.email_verified_at.is_some())
    }

    fn into_resolved(self) -> ResolvedSnagUser {
        let email_verified = self.email_verified();
        ResolvedSnagUser {
            id: self.id,
            wallet_address: self.wallet_address,
            email_verified,
        }
    }
}

fn canonical_wallet_address(
    resolved_user: Option<&ResolvedSnagUser>,
    wallet_address: Option<String>,
) -> String {
    resolved_user
        .and_then(|user| user.wallet_address.clone())
        .or(wallet_address)
        .unwrap_or_else(generated_wallet_address)
}

fn is_supported_user_metadata_bad_request(status: reqwest::StatusCode, body: &str) -> bool {
    if status != reqwest::StatusCode::BAD_REQUEST {
        return false;
    }

    serde_json::from_str::<SnagUserMetadataResponse>(body).is_ok_and(|response| {
        response.success == Some(false)
            && response.message.as_deref() == Some("User already exists")
    })
}

fn is_supported_complete_rule_bad_request(status: reqwest::StatusCode, body: &str) -> bool {
    if status != reqwest::StatusCode::BAD_REQUEST {
        return false;
    }

    serde_json::from_str::<SnagRuleCompleteResponse>(body).is_ok_and(|response| {
        response.rewarded == Some(true)
            && response.message.as_deref() == Some("You have already been rewarded")
    })
}

fn is_accepted_user_metadata_response(status: reqwest::StatusCode, body: &str) -> bool {
    status.is_success()
        || status == reqwest::StatusCode::CONFLICT
        || is_supported_user_metadata_bad_request(status, body)
}

fn is_accepted_complete_rule_response(status: reqwest::StatusCode, body: &str) -> bool {
    status.is_success()
        || status == reqwest::StatusCode::CONFLICT
        || is_supported_complete_rule_bad_request(status, body)
}

fn is_accepted_connect_user_response(status: reqwest::StatusCode) -> bool {
    status.is_success() || status == reqwest::StatusCode::CONFLICT
}

async fn find_users_by_email(
    client: &Client,
    config: &SnagConfig,
    email: &str,
) -> anyhow::Result<Vec<SnagUserLookup>> {
    let response = client
        .get(format!("{}/api/users", config.base_url))
        .header("X-API-KEY", &config.api_key)
        .query(&[
            ("emailAddress", email),
            ("websiteId", config.website_id.as_str()),
            ("organizationId", config.organization_id.as_str()),
            ("limit", "20"),
        ])
        .send()
        .await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(anyhow!(
            "Snag email lookup failed with status {}: {}",
            status,
            response_body
        ));
    }

    serde_json::from_str::<SnagUsersResponse>(&response_body)
        .map(|response| response.data)
        .map_err(|error| {
            anyhow!(
                "failed to parse Snag email lookup response: {}; body: {}",
                error,
                response_body
            )
        })
}

async fn find_user_by_external_identifier(
    client: &Client,
    config: &SnagConfig,
    user_id: Uuid,
) -> anyhow::Result<Option<SnagUserLookup>> {
    let external_identifier = external_identifier(user_id);
    let response = client
        .get(format!("{}/api/users", config.base_url))
        .header("X-API-KEY", &config.api_key)
        .query(&[
            ("externalIdentifier", external_identifier.as_str()),
            ("websiteId", config.website_id.as_str()),
            ("organizationId", config.organization_id.as_str()),
            ("limit", "2"),
        ])
        .send()
        .await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(anyhow!(
            "Snag user lookup failed with status {}: {}",
            status,
            response_body
        ));
    }

    let mut parsed =
        serde_json::from_str::<SnagUsersResponse>(&response_body).map_err(|error| {
            anyhow!(
                "failed to parse Snag user lookup response: {}; body: {}",
                error,
                response_body
            )
        })?;
    match parsed.data.len() {
        0 => Ok(None),
        1 => Ok(parsed.data.pop()),
        count => Err(anyhow!(
            "expected at most one Snag user for external identifier {}, got {}",
            external_identifier,
            count
        )),
    }
}

async fn resolve_snag_user(
    client: &Client,
    config: &SnagConfig,
    user_id: Uuid,
    email: &str,
) -> anyhow::Result<Option<ResolvedSnagUser>> {
    let email_matches = find_users_by_email(client, config, email).await?;
    if let Some(user) = email_matches.iter().find(|user| user.email_verified()) {
        return Ok(Some(user.clone().into_resolved()));
    }
    if let Some(user) = email_matches.into_iter().next() {
        return Ok(Some(user.into_resolved()));
    }

    Ok(find_user_by_external_identifier(client, config, user_id)
        .await?
        .map(SnagUserLookup::into_resolved))
}

async fn resolve_snag_user_by_email(
    client: &Client,
    config: &SnagConfig,
    email: &str,
) -> anyhow::Result<Option<ResolvedSnagUser>> {
    let email_matches = find_users_by_email(client, config, email).await?;
    if let Some(user) = email_matches.iter().find(|user| user.email_verified()) {
        return Ok(Some(user.clone().into_resolved()));
    }

    Ok(email_matches
        .into_iter()
        .next()
        .map(SnagUserLookup::into_resolved))
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
    if is_accepted_user_metadata_response(status, &response_body) {
        if status == reqwest::StatusCode::CONFLICT {
            tracing::warn!("Snag user upsert returned conflict: {}", response_body);
        } else if is_supported_user_metadata_bad_request(status, &response_body) {
            tracing::warn!(
                "Snag user upsert returned supported bad request response: {}",
                response_body
            );
        }
        return Ok(());
    }
    Err(anyhow!(
        "Snag user upsert failed with status {}: {}",
        status,
        response_body
    ))
}

async fn send_connect_user(
    client: &Client,
    config: &SnagConfig,
    user_id: Uuid,
    wallet_address: &str,
    verification: SnagWalletVerification,
) -> anyhow::Result<()> {
    let response = client
        .post(format!("{}/api/users/connect", config.base_url))
        .header("X-API-KEY", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&build_connect_user_request(
            config,
            user_id,
            wallet_address.to_string(),
            verification,
        ))
        .send()
        .await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if is_accepted_connect_user_response(status) {
        if status == reqwest::StatusCode::CONFLICT {
            tracing::warn!("Snag user connect returned conflict: {}", response_body);
        }
        return Ok(());
    }

    Err(anyhow!(
        "Snag user connect failed with status {}: {}",
        status,
        response_body
    ))
}

async fn send_complete_rule(
    client: &Client,
    config: &SnagConfig,
    rule: SnagRuleKind,
    target: SnagRuleTarget,
) -> anyhow::Result<()> {
    let request = client
        .post(format!(
            "{}/api/loyalty/rules/{}/complete",
            config.base_url,
            rule.external_rule_id(config)
        ))
        .header("X-API-KEY", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&match target {
            SnagRuleTarget::Wallet(wallet_address) => SnagRuleCompleteRequest {
                wallet_address: Some(wallet_address),
                user_id: None,
            },
            SnagRuleTarget::User(user_id) => SnagRuleCompleteRequest {
                wallet_address: None,
                user_id: Some(user_id),
            },
        });
    let response = request.send().await?;
    let status = response.status();
    let response_body = response.text().await.unwrap_or_default();
    if is_accepted_complete_rule_response(status, &response_body) {
        if status == reqwest::StatusCode::CONFLICT {
            tracing::warn!(
                "Snag loyalty completion returned conflict: {}",
                response_body
            );
        } else if is_supported_complete_rule_bad_request(status, &response_body) {
            tracing::warn!(
                "Snag loyalty completion returned supported bad request response: {}",
                response_body
            );
        }
        return Ok(());
    }
    Err(anyhow!(
        "Snag loyalty completion failed with status {}: {}",
        status,
        response_body
    ))
}

#[tracing::instrument(name = "snag_sync_user_metadata", skip(client, config, wallet_address))]
pub async fn sync_user_metadata(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: String,
) -> anyhow::Result<()> {
    let body = build_user_metadata_request(user_id, email, wallet_address);
    send_create_or_update_user(&client, &config, &body).await
}

#[tracing::instrument(name = "snag_sync_confirmed_email", skip(client, config))]
pub async fn sync_confirmed_email(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: Option<String>,
) -> anyhow::Result<()> {
    let resolved_user = resolve_snag_user(&client, &config, user_id, &email).await?;
    let wallet_address = canonical_wallet_address(resolved_user.as_ref(), wallet_address);

    sync_user_metadata(client, config, user_id, email, wallet_address).await
}

#[tracing::instrument(name = "snag_sync_registered_email_reward", skip(client, config))]
pub async fn sync_registered_email_reward(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: Option<String>,
) -> anyhow::Result<SnagEmailRewardOutcome> {
    let resolved_user = resolve_snag_user(&client, &config, user_id, &email).await?;

    if let Some(resolved_user) = resolved_user {
        if resolved_user.email_verified {
            send_complete_rule(
                &client,
                &config,
                SnagRuleKind::EmailRegistered,
                SnagRuleTarget::User(resolved_user.id),
            )
            .await?;
            return Ok(SnagEmailRewardOutcome::Consumed);
        }

        return Ok(SnagEmailRewardOutcome::Pending);
    }

    let wallet_address = wallet_address.unwrap_or_else(generated_wallet_address);
    sync_user_metadata(client, config, user_id, email, wallet_address).await?;
    Ok(SnagEmailRewardOutcome::Pending)
}

#[tracing::instrument(
    name = "snag_sync_connected_wallet",
    skip(client, config, wallet_address, verification)
)]
pub async fn sync_connected_wallet(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: String,
    verification: SnagWalletVerification,
) -> anyhow::Result<()> {
    let resolved_user = resolve_snag_user(&client, &config, user_id, &email).await?;
    let target = match resolved_user {
        Some(existing_user)
            if existing_user.wallet_address.as_deref() == Some(wallet_address.as_str()) =>
        {
            SnagRuleTarget::User(existing_user.id)
        }
        Some(existing_user) => {
            send_connect_user(
                &client,
                &config,
                existing_user.id,
                &wallet_address,
                verification,
            )
            .await?;
            SnagRuleTarget::User(existing_user.id)
        }
        None => {
            sync_user_metadata(
                client.clone(),
                config.clone(),
                user_id,
                email,
                wallet_address.clone(),
            )
            .await?;
            SnagRuleTarget::Wallet(wallet_address.clone())
        }
    };

    complete_wallet_rule(client, config, target).await
}

#[tracing::instrument(name = "snag_complete_extension_rule", skip(client, config, target))]
async fn complete_extension_rule(
    client: Client,
    config: SnagConfig,
    target: SnagRuleTarget,
) -> anyhow::Result<()> {
    send_complete_rule(&client, &config, SnagRuleKind::Extension, target).await
}

#[tracing::instrument(name = "snag_complete_wallet_rule", skip(client, config, target))]
async fn complete_wallet_rule(
    client: Client,
    config: SnagConfig,
    target: SnagRuleTarget,
) -> anyhow::Result<()> {
    send_complete_rule(&client, &config, SnagRuleKind::Wallet, target).await
}

#[tracing::instrument(name = "snag_sync_first_activation", skip(client, config))]
pub async fn sync_first_activation(
    client: Client,
    config: SnagConfig,
    user_id: Uuid,
    email: String,
    wallet_address: Option<String>,
) -> anyhow::Result<SnagFirstActivationOutcome> {
    if let Some(existing_user) = resolve_snag_user_by_email(&client, &config, &email).await? {
        complete_extension_rule(client, config, SnagRuleTarget::User(existing_user.id)).await?;
        return Ok(SnagFirstActivationOutcome::Consumed);
    }

    let wallet_address = wallet_address.unwrap_or_else(generated_wallet_address);
    sync_user_metadata(
        client.clone(),
        config.clone(),
        user_id,
        email,
        wallet_address.clone(),
    )
    .await?;
    Ok(SnagFirstActivationOutcome::Pending)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(server: &MockServer) -> SnagConfig {
        SnagConfig {
            base_url: server.uri(),
            api_key: "api-key".to_string(),
            external_rule_email_registered: "email-registered-rule".to_string(),
            external_rule_extension: "extension-rule".to_string(),
            external_rule_wallet: "wallet-rule".to_string(),
            external_rule_mobile: "mobile-rule".to_string(),
            website_id: Uuid::from_u128(1).to_string(),
            organization_id: Uuid::from_u128(2).to_string(),
        }
    }

    fn request_body_json(body: &[u8]) -> Value {
        serde_json::from_slice(body).unwrap()
    }

    async fn mount_email_lookup(
        server: &MockServer,
        config: &SnagConfig,
        email: &str,
        body: Value,
    ) {
        Mock::given(method("GET"))
            .and(path("/api/users"))
            .and(query_param("emailAddress", email))
            .and(query_param("websiteId", config.website_id.clone()))
            .and(query_param(
                "organizationId",
                config.organization_id.clone(),
            ))
            .and(query_param("limit", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(server)
            .await;
    }

    async fn mount_external_lookup(
        server: &MockServer,
        config: &SnagConfig,
        user_id: Uuid,
        body: Value,
    ) {
        Mock::given(method("GET"))
            .and(path("/api/users"))
            .and(query_param("externalIdentifier", user_id.to_string()))
            .and(query_param("websiteId", config.website_id.clone()))
            .and(query_param(
                "organizationId",
                config.organization_id.clone(),
            ))
            .and(query_param("limit", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(server)
            .await;
    }

    #[test]
    fn serializes_user_metadata_payload_in_camel_case() {
        let payload = SnagUserMetadataRequest {
            id: "user-id".to_string(),
            external_identifier: "user-id".to_string(),
            wallet_address: "wallet".to_string(),
            email_address: "user@example.com".to_string(),
            display_name: "user@example.com".to_string(),
            user_group_external_identifier: "group-id".to_string(),
        };

        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["id"], "user-id");
        assert_eq!(json["externalIdentifier"], "user-id");
        assert_eq!(json["walletAddress"], "wallet");
        assert_eq!(json["emailAddress"], "user@example.com");
        assert_eq!(json["displayName"], "user@example.com");
        assert_eq!(json["userGroupExternalIdentifier"], "group-id");
    }

    #[test]
    fn serializes_connect_user_payload_with_optional_verification_fields() {
        let config = SnagConfig {
            base_url: "https://snag.example.com".to_string(),
            api_key: "api-key".to_string(),
            external_rule_email_registered: "email-registered-rule".to_string(),
            external_rule_extension: "extension-rule".to_string(),
            external_rule_wallet: "wallet-rule".to_string(),
            external_rule_mobile: "mobile-rule".to_string(),
            website_id: "website-id".to_string(),
            organization_id: "organization-id".to_string(),
        };
        let payload = build_connect_user_request(
            &config,
            Uuid::nil(),
            "wallet".to_string(),
            SnagWalletVerification::verified_locally_only(),
        );

        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["websiteId"], "website-id");
        assert_eq!(json["organizationId"], "organization-id");
        assert_eq!(json["walletType"], "solana");
        assert_eq!(json["walletAddress"], "wallet");
        assert_eq!(json["userId"], Uuid::nil().to_string());
        assert_eq!(json["confirmDisconnect"], true);
        assert_eq!(json["verificationData"]["verifiedLocally"], true);
        assert!(json["verificationData"].get("message").is_none());
        assert!(json["verificationData"].get("signature").is_none());
    }

    #[test]
    fn serializes_complete_rule_payload_with_user_id() {
        let payload = SnagRuleCompleteRequest {
            wallet_address: None,
            user_id: Some(Uuid::nil()),
        };

        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["userId"], Uuid::nil().to_string());
        assert!(json.get("walletAddress").is_none());
    }

    #[test]
    fn supports_duplicate_user_metadata_bad_request() {
        assert!(is_supported_user_metadata_bad_request(
            reqwest::StatusCode::BAD_REQUEST,
            r#"{"success": false, "message": "User already exists"}"#
        ));
    }

    #[test]
    fn supports_already_rewarded_complete_rule_bad_request() {
        assert!(is_supported_complete_rule_bad_request(
            reqwest::StatusCode::BAD_REQUEST,
            r#"{"message": "You have already been rewarded", "rewarded": true}"#
        ));
    }

    #[test]
    fn rejects_unknown_user_metadata_bad_request() {
        assert!(!is_supported_user_metadata_bad_request(
            reqwest::StatusCode::BAD_REQUEST,
            r#"{"success": false, "message": "Something else"}"#
        ));
    }

    #[test]
    fn rejects_unrewarded_complete_rule_bad_request() {
        assert!(!is_supported_complete_rule_bad_request(
            reqwest::StatusCode::BAD_REQUEST,
            r#"{"message": "You have already been rewarded", "rewarded": false}"#
        ));
    }

    #[test]
    fn accepts_complete_rule_queued_success_response() {
        assert!(is_accepted_complete_rule_response(
            reqwest::StatusCode::OK,
            r#"{"message": "Completion request added to queue", "data": {}}"#
        ));
    }

    #[test]
    fn selects_expected_external_rule_ids() {
        let config = SnagConfig {
            base_url: "https://snag.example.com".to_string(),
            api_key: "api-key".to_string(),
            external_rule_email_registered: "email-registered-rule".to_string(),
            external_rule_extension: "extension-rule".to_string(),
            external_rule_wallet: "wallet-rule".to_string(),
            external_rule_mobile: "mobile-rule".to_string(),
            website_id: "website-id".to_string(),
            organization_id: "organization-id".to_string(),
        };

        assert_eq!(
            SnagRuleKind::EmailRegistered.external_rule_id(&config),
            "email-registered-rule"
        );
        assert_eq!(
            SnagRuleKind::Extension.external_rule_id(&config),
            "extension-rule"
        );
        assert_eq!(
            SnagRuleKind::Wallet.external_rule_id(&config),
            "wallet-rule"
        );
    }

    #[test]
    fn snag_cutoff_date_uses_default_when_env_is_missing() {
        assert_eq!(
            resolve_snag_cutoff_date(None),
            utc_midnight(2026, Month::March, 24)
        );
    }

    #[test]
    fn snag_cutoff_date_parses_env_override() {
        assert_eq!(
            resolve_snag_cutoff_date(Some("2026-03-25")),
            utc_midnight(2026, Month::March, 25)
        );
    }

    #[test]
    fn snag_cutoff_date_falls_back_to_default_when_env_is_invalid() {
        assert_eq!(
            resolve_snag_cutoff_date(Some("march-24-2026")),
            utc_midnight(2026, Month::March, 24)
        );
    }

    #[test]
    fn users_created_on_default_snag_rollout_day_are_eligible() {
        let created_at = utc_midnight(2026, Month::March, 24);

        assert!(created_at >= resolve_snag_cutoff_date(None));
    }

    #[test]
    fn users_created_before_default_snag_rollout_day_are_not_eligible() {
        let created_at = utc_midnight(2026, Month::March, 23);

        assert!(created_at < resolve_snag_cutoff_date(None));
    }

    #[test]
    fn generated_wallet_address_looks_like_a_solana_pubkey() {
        let wallet = generated_wallet_address();
        assert!(!wallet.is_empty());
        assert!(wallet.len() >= 32);
    }

    #[tokio::test]
    async fn find_user_by_external_identifier_errors_when_multiple_users_are_returned() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);

        Mock::given(method("GET"))
            .and(path("/api/users"))
            .and(query_param("externalIdentifier", user_id.to_string()))
            .and(query_param("websiteId", config.website_id.clone()))
            .and(query_param(
                "organizationId",
                config.organization_id.clone(),
            ))
            .and(query_param("limit", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": Uuid::from_u128(10),
                        "walletAddress": "wallet-a"
                    },
                    {
                        "id": Uuid::from_u128(11),
                        "walletAddress": "wallet-b"
                    }
                ],
                "hasNextPage": false
            })))
            .mount(&server)
            .await;

        let error = find_user_by_external_identifier(&client, &config, user_id)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("expected at most one Snag user"));
    }

    #[tokio::test]
    async fn sync_first_activation_creates_placeholder_user_when_missing_and_leaves_pending() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/metadatas"))
            .and(header("x-api-key", "api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ok"})))
            .mount(&server)
            .await;

        let outcome = sync_first_activation(
            client,
            config.clone(),
            user_id,
            "user@example.com".to_string(),
            None,
        )
        .await
        .unwrap();
        assert_eq!(outcome, SnagFirstActivationOutcome::Pending);

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 2);

        let metadata_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/users/metadatas")
            .unwrap();
        let metadata_body = request_body_json(&metadata_request.body);
        let created_wallet = metadata_body["walletAddress"].as_str().unwrap().to_string();
        assert!(!created_wallet.is_empty());
        assert_eq!(metadata_body["externalIdentifier"], user_id.to_string());
        assert_eq!(
            metadata_body["userGroupExternalIdentifier"],
            format!("bm-user-group:{user_id}")
        );
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/loyalty/rules/extension-rule/complete"));
    }

    #[tokio::test]
    async fn sync_first_activation_completes_rule_for_existing_email_user() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);
        let snag_user_id = Uuid::from_u128(10);

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({
                "data": [
                    {
                        "id": snag_user_id,
                        "walletAddress": "wallet-a"
                    }
                ],
                "hasNextPage": false
            }),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/loyalty/rules/extension-rule/complete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Completion request added to queue",
                "data": {}
            })))
            .mount(&server)
            .await;

        let outcome = sync_first_activation(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            Some("real-wallet".to_string()),
        )
        .await
        .unwrap();
        assert_eq!(outcome, SnagFirstActivationOutcome::Consumed);

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 2);
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/users/metadatas"));

        let complete_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/loyalty/rules/extension-rule/complete")
            .unwrap();
        let complete_body = request_body_json(&complete_request.body);
        assert_eq!(complete_body["userId"], snag_user_id.to_string());
        assert!(complete_body.get("walletAddress").is_none());
    }

    #[tokio::test]
    async fn sync_confirmed_email_creates_placeholder_user_when_missing() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(52);

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/metadatas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ok"})))
            .mount(&server)
            .await;

        sync_confirmed_email(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            None,
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 3);
        let metadata_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/users/metadatas")
            .unwrap();
        let metadata_body = request_body_json(&metadata_request.body);
        assert_eq!(metadata_body["externalIdentifier"], user_id.to_string());
        assert_eq!(metadata_body["emailAddress"], "user@example.com");
        assert!(!metadata_body["walletAddress"]
            .as_str()
            .unwrap_or_default()
            .is_empty());
    }

    #[tokio::test]
    async fn sync_confirmed_email_reuses_existing_snag_wallet() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(62);
        let placeholder_wallet = "placeholder-wallet";

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({
                "data": [
                    {
                        "id": Uuid::from_u128(12),
                        "walletAddress": placeholder_wallet
                    }
                ],
                "hasNextPage": false
            }),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/metadatas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ok"})))
            .mount(&server)
            .await;

        sync_confirmed_email(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            Some("real-wallet".to_string()),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 3);
        let metadata_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/users/metadatas")
            .unwrap();
        let metadata_body = request_body_json(&metadata_request.body);
        assert_eq!(metadata_body["walletAddress"], placeholder_wallet);
    }

    #[tokio::test]
    async fn sync_registered_email_reward_completes_with_first_verified_email_match() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(72);
        let verified_user_id = Uuid::from_u128(73);

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({
                "data": [
                    {
                        "id": Uuid::from_u128(71),
                        "emailVerifiedAt": null
                    },
                    {
                        "id": verified_user_id,
                        "emailVerifiedAt": "2026-03-25T00:00:00Z"
                    }
                ],
                "hasNextPage": false
            }),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/loyalty/rules/email-registered-rule/complete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Completion request added to queue",
                "data": {}
            })))
            .mount(&server)
            .await;

        let outcome = sync_registered_email_reward(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            None,
        )
        .await
        .unwrap();

        assert_eq!(outcome, SnagEmailRewardOutcome::Consumed);
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 2);
        let complete_request = requests
            .iter()
            .find(|request| {
                request.url.path() == "/api/loyalty/rules/email-registered-rule/complete"
            })
            .unwrap();
        let complete_body = request_body_json(&complete_request.body);
        assert_eq!(complete_body["userId"], verified_user_id.to_string());
    }

    #[tokio::test]
    async fn sync_registered_email_reward_creates_metadata_and_stays_pending_when_missing() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(82);

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/metadatas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ok"})))
            .mount(&server)
            .await;

        let outcome = sync_registered_email_reward(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            None,
        )
        .await
        .unwrap();

        assert_eq!(outcome, SnagEmailRewardOutcome::Pending);
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 3);
        assert!(!requests.iter().any(|request| {
            request.url.path() == "/api/loyalty/rules/email-registered-rule/complete"
        }));
    }

    #[tokio::test]
    async fn sync_connected_wallet_creates_user_when_missing() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);
        let wallet_address = "real-wallet";

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/metadatas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "ok"})))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/api/loyalty/rules/wallet-rule/complete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Completion request added to queue",
                "data": {}
            })))
            .mount(&server)
            .await;

        sync_connected_wallet(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            wallet_address.to_string(),
            SnagWalletVerification::verified_locally_only(),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 4);
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/users/connect"));

        let metadata_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/users/metadatas")
            .unwrap();
        let metadata_body = request_body_json(&metadata_request.body);
        assert_eq!(metadata_body["walletAddress"], wallet_address);
        assert_eq!(metadata_body["externalIdentifier"], user_id.to_string());
        assert_eq!(
            metadata_body["userGroupExternalIdentifier"],
            format!("bm-user-group:{user_id}")
        );

        let complete_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/loyalty/rules/wallet-rule/complete")
            .unwrap();
        let complete_body = request_body_json(&complete_request.body);
        assert_eq!(complete_body["walletAddress"], wallet_address);
    }

    #[tokio::test]
    async fn sync_connected_wallet_connects_real_wallet_to_existing_user_group() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);
        let existing_user_id = Uuid::from_u128(77);
        let wallet_address = "real-wallet";

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({
                "data": [
                    {
                        "id": existing_user_id,
                        "walletAddress": "placeholder-wallet"
                    }
                ],
                "hasNextPage": false
            }),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/users/connect"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": existing_user_id,
                "walletAddress": wallet_address,
                "createdAt": "2026-03-25T00:00:00Z",
                "updatedAt": "2026-03-25T00:00:00Z"
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/api/loyalty/rules/wallet-rule/complete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Completion request added to queue",
                "data": {}
            })))
            .mount(&server)
            .await;

        sync_connected_wallet(
            client,
            config.clone(),
            user_id,
            "user@example.com".to_string(),
            wallet_address.to_string(),
            SnagWalletVerification::verified_solana(
                "signed-message".to_string(),
                "signed-signature".to_string(),
            ),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 4);
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/users/metadatas"));

        let connect_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/users/connect")
            .unwrap();
        let connect_body = request_body_json(&connect_request.body);
        assert_eq!(connect_body["websiteId"], config.website_id);
        assert_eq!(connect_body["organizationId"], config.organization_id);
        assert_eq!(connect_body["walletType"], "solana");
        assert_eq!(connect_body["walletAddress"], wallet_address);
        assert_eq!(connect_body["userId"], existing_user_id.to_string());
        assert_eq!(connect_body["confirmDisconnect"], true);
        assert_eq!(
            connect_body["verificationData"]["message"],
            "signed-message"
        );
        assert_eq!(
            connect_body["verificationData"]["signature"],
            "signed-signature"
        );
        assert_eq!(connect_body["verificationData"]["verifiedLocally"], true);

        let complete_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/loyalty/rules/wallet-rule/complete")
            .unwrap();
        let complete_body = request_body_json(&complete_request.body);
        assert_eq!(complete_body["userId"], existing_user_id.to_string());
        assert!(complete_body.get("walletAddress").is_none());
    }

    #[tokio::test]
    async fn sync_connected_wallet_skips_connect_when_wallet_already_matches() {
        let server = MockServer::start().await;
        let config = test_config(&server);
        let client = Client::new();
        let user_id = Uuid::from_u128(42);
        let wallet_address = "real-wallet";

        mount_email_lookup(
            &server,
            &config,
            "user@example.com",
            json!({"data": [], "hasNextPage": false}),
        )
        .await;
        mount_external_lookup(
            &server,
            &config,
            user_id,
            json!({
                "data": [
                    {
                        "id": Uuid::from_u128(77),
                        "walletAddress": wallet_address
                    }
                ],
                "hasNextPage": false
            }),
        )
        .await;
        Mock::given(method("POST"))
            .and(path("/api/loyalty/rules/wallet-rule/complete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Completion request added to queue",
                "data": {}
            })))
            .mount(&server)
            .await;

        sync_connected_wallet(
            client,
            config,
            user_id,
            "user@example.com".to_string(),
            wallet_address.to_string(),
            SnagWalletVerification::verified_locally_only(),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 3);
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/users/connect"));
        assert!(!requests
            .iter()
            .any(|request| request.url.path() == "/api/users/metadatas"));

        let complete_request = requests
            .iter()
            .find(|request| request.url.path() == "/api/loyalty/rules/wallet-rule/complete")
            .unwrap();
        let complete_body = request_body_json(&complete_request.body);
        assert_eq!(complete_body["userId"], Uuid::from_u128(77).to_string());
        assert!(complete_body.get("walletAddress").is_none());
    }
}
