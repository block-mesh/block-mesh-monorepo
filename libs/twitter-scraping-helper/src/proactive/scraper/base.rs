use crate::{auth, config::Config};
use anyhow::anyhow;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderName};
use reqwest::Client;
use secrecy::{ExposeSecret as _, SecretString};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct Scraper {
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
    pub async fn incr_tweets_collected(&self, incr: u64) {
        *self.tweets_collected.write().await += incr;
    }

    #[tracing::instrument(name = "get_tweets_collected", skip_all)]
    pub async fn get_tweets_collected(&self) -> u64 {
        *self.tweets_collected.read().await
    }

    #[tracing::instrument(name = "new", skip_all)]
    pub fn new(client: Client, csrf: SecretString, config: Config) -> Self {
        Self {
            min_sleep: 1_000,
            limit: Arc::new(RwLock::new(Default::default())),
            remaining: Arc::new(RwLock::new(Default::default())),
            reset: Arc::new(RwLock::new(Default::default())),
            client,
            bearer_token: config.bearer_token,
            csrf,
            tweets_collected: Arc::new(Default::default()),
        }
    }

    #[tracing::instrument(name = "wait_for_reset", skip_all)]
    pub async fn wait_for_reset(&self) {
        let now = Utc::now();
        let remaining = *self.remaining.read().await;
        let reset = *self.reset.read().await;
        let limit = *self.limit.read().await;
        if remaining <= 0 && reset > now.timestamp() {
            let diff = reset - now.timestamp();
            tracing::info!(
                "Sleeping for {}[sec] | limit = {} | remaining = {} | reset = {} | now = {} | total_collected = {}",
                diff,
                limit,
                remaining,
                reset,
                now.timestamp(),
                self.get_tweets_collected().await
            );
            sleep(Duration::from_secs(diff as u64)).await;
        } else {
            tracing::info!(
                "Sleeping min for {}[ms] | limit = {} | remaining = {} | reset = {} | now = {} | total_collected = {}",
                self.min_sleep,
                limit,
                remaining,
                reset,
                now.timestamp(),
                self.get_tweets_collected().await
            );
            sleep(Duration::from_millis(self.min_sleep)).await;
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
                    *self.limit.write().await = v;
                }
                "x-rate-limit-remaining" => {
                    let v = value.parse::<i32>().unwrap_or(1);
                    *self.remaining.write().await = v;
                }
                "x-rate-limit-reset" => {
                    let v = value.parse::<i64>().unwrap_or(1);
                    *self.reset.write().await = v;
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
        let (client, csrf) = auth::from_config(&config).await?;
        Ok(Self::new(client, csrf, config))
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
