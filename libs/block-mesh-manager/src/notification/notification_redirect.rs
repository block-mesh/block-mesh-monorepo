use axum::response::Redirect;
use url::Url;

pub struct NotificationRedirect;

impl NotificationRedirect {
    pub fn redirect(summary: &str, detailed: &str, go_to: &str) -> Redirect {
        let mut url = Url::parse("tmp://notification").unwrap();
        url.query_pairs_mut()
            .append_pair("summary", summary)
            .append_pair("go_to", go_to)
            .append_pair("detailed", detailed);
        let url = url.to_string();
        let url = url.split("tmp:/").last().unwrap();
        Redirect::to(url)
    }
}
