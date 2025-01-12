use anyhow::{anyhow, Context};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    User {
        username: String,
        password: SecretString,
        email: Option<String>,
    },
    Cookie {
        cookie: SecretString,
    },
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: reqwest::Url,
    pub bearer_token: SecretString,
    pub auth: AuthConfig,
}

#[tracing::instrument(name = "via_envars", ret, err)]
pub fn via_envars() -> anyhow::Result<Config> {
    let mode = env::var("TWITTER_SCRAPER_MODE")?;
    let base = reqwest::Url::parse(&env::var("TWITTER_SCRAPER_URL").unwrap_or_default())?;
    let bearer_token = SecretString::new(Box::from(
        env::var("TWITTER_SCRAPER_BEARER_TOKEN").unwrap_or_default(),
    ));
    if bearer_token.expose_secret().is_empty() {
        return Err(anyhow!("Missing TWITTER_SCRAPER_BEARER_TOKEN"));
    }
    if mode == "cookie" {
        let cookie_str = SecretString::new(Box::from(
            env::var("TWITTER_SCRAPER_COOKIE").unwrap_or_default(),
        ));
        if cookie_str.expose_secret().is_empty() {
            return Err(anyhow!("Missing TWITTER_SCRAPER_COOKIE"));
        }

        let cookie = AuthConfig::Cookie { cookie: cookie_str };

        Ok(Config {
            base,
            bearer_token,
            auth: cookie,
        })
    } else if mode == "user" {
        let username = env::var("TWITTER_SCRAPER_USERNAME").unwrap_or_default();
        if username.is_empty() {
            return Err(anyhow!("Missing TWITTER_SCRAPER_USERNAME"));
        }
        let email = env::var("TWITTER_SCRAPER_EMAIL").ok();
        let password = SecretString::new(Box::from(
            env::var("TWITTER_SCRAPER_PASSWORD").unwrap_or_default(),
        ));
        if password.expose_secret().is_empty() {
            return Err(anyhow!("Missing TWITTER_SCRAPER_PASSWORD"));
        }
        let user = AuthConfig::User {
            username,
            email,
            password,
        };
        Ok(Config {
            base,
            bearer_token,
            auth: user,
        })
    } else {
        Err(anyhow!("Unknown TWITTER_SCRAPER_MODE {}", mode))
    }
}

#[allow(dead_code)]
pub fn load(path: &str) -> anyhow::Result<Config> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading `{path}`"))?;
    toml::from_str(&text).map_err(Into::into)
}
