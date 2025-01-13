use crate::proactive::scraper::base::Scraper;
use crate::proactive::types::timeline::search::Product;
use crate::proactive::types::timeline::v1::QueryTweetsResponse;
use anyhow::anyhow;
use chrono::NaiveDate;

impl Scraper {
    #[tracing::instrument(name = "tweets", skip_all)]
    pub async fn tweets(
        &self,
        search_mode: &Product,
        query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<QueryTweetsResponse> {
        self.tweet_timeline(search_mode, query, count, cursor)
            .await
            .map(|timeline| crate::proactive::types::timeline::search::parse_tweets(&timeline))
    }

    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(name = "scrape_tweets", skip_all, ret, err)]
    pub async fn scrape_tweets(
        &self,
        from: &str,
        since: &NaiveDate,
        until: &NaiveDate,
        count: u32,
        input_cursor: Option<String>,
    ) -> anyhow::Result<QueryTweetsResponse> {
        let mut query = "".to_string();
        query.push_str(&format!("(from:{}) ", from));
        query.push_str(&format!("since:{} ", since));
        query.push_str(&format!("until:{}", until));
        let mut cursor = input_cursor;
        let mut tweets = QueryTweetsResponse::default();
        loop {
            let new = self
                .tweets(&Product::Top, &query, count, cursor)
                .await
                .map_err(|e| {
                    tracing::error!("scraper.tweets error {}", e);
                    anyhow!("failed to scrape tweets")
                })?;
            tracing::info!(
                "Got {} more tweets, total collected {}",
                new.tweets.len(),
                self.get_tweets_collected()
            );
            if new.tweets.is_empty() {
                break;
            }
            self.incr_tweets_collected(new.tweets.len() as u64);
            cursor = new.next;
            tracing::info!("cursor = {}", cursor.clone().unwrap_or_default());
            tweets.merge(new.tweets);
        }
        Ok(tweets)
    }
}
