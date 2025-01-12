use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: reqwest::Url,
    pub bearer_token: SecretString, // Bearer token
    pub cookie: SecretString,       // x-csrf-token
}

impl Config {
    pub fn new(bearer_token: &str, cookie: &str, base: &str) -> anyhow::Result<Self> {
        let base = reqwest::Url::parse(base)?;
        let bearer_token = SecretString::new(Box::from(bearer_token));
        let cookie = SecretString::new(Box::from(cookie));
        Ok(Self {
            base,
            bearer_token,
            cookie,
        })
    }
}
