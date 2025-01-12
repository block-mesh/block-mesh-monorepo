use crate::proactive::scraper::base::Scraper;
use crate::proactive::types::timeline::search::{Product, SearchTimelineParams};
use anyhow::anyhow;
use secrecy::ExposeSecret;
use std::thread::sleep;
use std::time::Duration;

impl Scraper {
    #[tracing::instrument(name = "tweet_timeline", skip_all)]
    pub async fn tweet_timeline(
        &self,
        search_mode: &Product,
        raw_query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::proactive::types::timeline::search::SearchTimeline> {
        let mut search_time_line_params = SearchTimelineParams::default();
        search_time_line_params.update_count(count);
        search_time_line_params.update_raw_query(raw_query);
        search_time_line_params.update_cursor(cursor.clone());
        search_time_line_params.update_product(search_mode.clone());
        let mut retries = 0;
        let params = search_time_line_params.params()?;
        let url = self.base.to_string();
        let request = self
            .client
            // .get("https://api.twitter.com/graphql/nK1dw4oV3k4w5TdtcAdSww/SearchTimeline")
            .get(&url)
            .query(&params)
            .bearer_auth(self.bearer_token.expose_secret())
            .header("x-csrf-token", self.csrf.expose_secret())
            .header("content-type", "application/json")
            .header("x-twitter-active-user", "yes")
            .header("x-twitter-auth-type", "OAuth2Session")
            .header("x-twitter-client-language", "en")
            .header("accept", "*/*")
            .build()?;
        let response = self.client.execute(request).await?;
        let headers = response.headers().clone();
        self.extract_headers(headers).await;
        let response = response.text().await?;
        let response = response.trim();
        if response.contains("Rate limit exceeded") {
            retries += 1;
            let dur = 3 * retries;
            tracing::info!("Sleeping due to rate limit for {} seconds", dur);
            sleep(Duration::from_secs(dur));
        } else if response.contains("Could not authenticate you") {
            return Err(anyhow!("Auth issues : {}", response));
        } else {
            return match serde_json::from_str::<
                crate::proactive::types::timeline::search::SearchTimeline,
            >(response)
            {
                Ok(json) => Ok(json),
                Err(e) => Err(anyhow!("Error {} | text {}", e, response)),
            };
        }
        Err(anyhow!("Out of retries"))
    }
}
