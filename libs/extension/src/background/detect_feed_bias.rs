use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedElement {
    origin: Option<String>,
    user_name: Option<String>,
    link: Option<String>,
    id: Option<String>,
    raw: Option<String>,
    valid: bool,
}

impl FeedElement {
    pub fn new(raw: String) -> Self {
        Self {
            user_name: None,
            link: None,
            id: None,
            raw: Some(raw),
            valid: false,
            origin: None,
        }
    }

    pub fn validate(&mut self) -> bool {
        if self.user_name.is_none()
            || self.link.is_none()
            || self.id.is_none()
            || self.raw.is_none()
            || self.origin.is_none()
        {
            false
        } else {
            self.valid = true;
            true
        }
    }
}

#[wasm_bindgen]
pub async fn read_dom(html: String, origin: String) {
    let fragment = Html::parse_fragment(&html);
    let href = Selector::parse("[href]").unwrap();
    let re = Regex::new(r"/(?P<username>[^/]+)/status/(?P<id>\d+$)").unwrap();
    let mut feed_element = FeedElement::new(html);
    for element in fragment.select(&href) {
        if let Some(href_value) = element.value().attr("href") {
            if let Some(caps) = re.captures(href_value) {
                if let Some(username) = caps.name("username") {
                    feed_element.user_name = Some(username.as_str().to_string());
                }
                if let Some(id) = caps.name("id") {
                    feed_element.id = Some(id.as_str().to_string());
                }
                feed_element.link = Some(href_value.to_string());
                feed_element.origin = Some(origin.clone());
            }
        }
    }
    if feed_element.validate() {}
}
