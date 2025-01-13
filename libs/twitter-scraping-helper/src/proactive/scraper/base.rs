use crate::proactive::config::Config;
use anyhow::anyhow;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::reqwest::http_client;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::Client;
use secrecy::{ExposeSecret as _, SecretString};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Scraper {
    pub base: Arc<String>,
    pub client: Client,
    pub bearer_token: SecretString,
    pub csrf: SecretString,
    pub limit: Arc<RwLock<i32>>,
    pub remaining: Arc<RwLock<i32>>,
    pub reset: Arc<RwLock<i64>>,
    pub tweets_collected: Arc<RwLock<u64>>,
    pub min_sleep: u64,
}

impl Scraper {
    #[tracing::instrument(name = "get_tweets_collected", skip_all)]
    pub fn incr_tweets_collected(&self, incr: u64) {
        if let Ok(mut v) = self.tweets_collected.write() {
            *v += incr;
        }
    }

    #[tracing::instrument(name = "get_tweets_collected", skip_all)]
    pub fn get_tweets_collected(&self) -> u64 {
        match self.tweets_collected.read() {
            Ok(v) => *v,
            Err(_) => 0,
        }
    }

    #[tracing::instrument(name = "new", skip_all)]
    pub fn new(client: Client, config: Config) -> Self {
        Self {
            base: Arc::new(config.base.to_string()),
            min_sleep: 1_000,
            limit: Arc::new(RwLock::new(Default::default())),
            remaining: Arc::new(RwLock::new(Default::default())),
            reset: Arc::new(RwLock::new(Default::default())),
            client,
            bearer_token: config.bearer_token,
            csrf: config.cookie,
            tweets_collected: Arc::new(Default::default()),
        }
    }

    #[tracing::instrument(name = "extract_headers", skip_all)]
    pub async fn extract_headers(&self, headers: HeaderMap) {
        for (key, value) in headers.into_iter() {
            let key = key
                .unwrap_or(HeaderName::from_static("tmp"))
                .as_str()
                .to_lowercase();
            let value = value.to_str().unwrap_or_default();
            match key.as_str() {
                "x-rate-limit-limit" => {
                    let v = value.parse::<i32>().unwrap_or(1);
                    if let Ok(mut limit) = self.limit.write() {
                        *limit = v;
                    }
                }
                "x-rate-limit-remaining" => {
                    let v = value.parse::<i32>().unwrap_or(1);
                    if let Ok(mut remaining) = self.remaining.write() {
                        *remaining = v;
                    }
                }
                "x-rate-limit-reset" => {
                    let v = value.parse::<i64>().unwrap_or(1);
                    if let Ok(mut reset) = self.reset.write() {
                        *reset = v;
                    }
                }
                _ => {}
            }
        }
    }

    #[tracing::instrument(name = "test_creds", skip_all)]
    pub async fn test_creds(&self) -> anyhow::Result<()> {
        let _ = self
            .client
            .get("https://x.com/i/api/graphql/dG4tvka_YkwFzagVLG6UDA/HomeTimeline")
            .bearer_auth(self.bearer_token.expose_secret())
            .header("x-csrf-token", self.csrf.expose_secret())
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Test creds failed due to = {}", e);
                anyhow!(e)
            })?;
        Ok(())
    }

    #[tracing::instrument(name = "from_config", err)]
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let client = http_client(DeviceType::Extension);
        Ok(Self::new(client, config))
    }
}

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum SearchMode {
    Top,
    Latest,
    Photos,
    Videos,
    Users,
}
