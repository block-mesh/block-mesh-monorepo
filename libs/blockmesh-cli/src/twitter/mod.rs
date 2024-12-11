use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub const USER_TWEETS: &str = "https://x.com/i/api/graphql/1mDAyxlBlMp6uokkzihecQ/UserTweets";

pub fn twitter_client() -> Client {
    ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .cookie_store(true)
        .no_hickory_dns()
        .use_rustls_tls()
        .build()
        .unwrap_or_default()
}
