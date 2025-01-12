use crate::proactive::scraper::base::Scraper;
use crate::proactive::types::timeline::search::Product;

impl Scraper {
    #[tracing::instrument(name = "tweets", skip_all)]
    pub async fn tweets(
        &self,
        search_mode: &Product,
        query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::proactive::types::timeline::v1::QueryTweetsResponse> {
        self.tweet_timeline(search_mode, query, count, cursor)
            .await
            .map(|timeline| crate::proactive::types::timeline::search::parse_tweets(&timeline))
    }
}
