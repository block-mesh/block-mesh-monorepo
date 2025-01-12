use crate::proactive::scraper::base::Scraper;
use crate::proactive::types::timeline::search::Product;

impl Scraper {
    #[tracing::instrument(name = "profiles", skip_all)]
    pub async fn profiles(
        &self,
        query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::proactive::types::timeline::v1::QueryProfilesResponse> {
        self.tweet_timeline(&Product::People, query, count, cursor)
            .await
            .map(|timeline| crate::proactive::types::timeline::search::parse_users(&timeline))
    }
}
