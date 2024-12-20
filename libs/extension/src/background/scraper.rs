/*
https://x.com/search?q=(from%3Atruth_terminal)%20until%3A2024-12-03%20since%3A2024-12-01&src=typed_query&f=top
 */
use crate::utils::log::log;
use chrono::NaiveDate;
use urlencoding::encode;

#[allow(dead_code)]
pub fn create_search_url(user_name: &str, until: NaiveDate, since: NaiveDate) -> String {
    let org = format!("(from:{}) until:{} since:{}", user_name, until, since);
    let org_encoded = encode(&org).to_string();
    let url = format!("https://x.com/search?q={}", org_encoded);
    log!("org {}", org);
    log!("org_encoded {}", org_encoded);
    log!("url {}", url);
    url
}
