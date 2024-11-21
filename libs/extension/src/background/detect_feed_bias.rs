use block_mesh_common::interfaces::server_api::FeedElement;
use regex::Regex;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn read_dom(html: String, origin: String) {
    let fragment = Html::parse_fragment(&html);
    let href = Selector::parse("[href]").unwrap();
    let re = Regex::new(r"/(?P<username>[^/]+)/status/(?P<id>\d+$)").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();

    for element in fragment.select(&href) {
        if let Some(href_value) = element.value().attr("href") {
            if let Some(caps) = re.captures(href_value) {
                if let Some(username) = caps.name("username") {
                    map.insert("user_name".to_owned(), username.as_str().to_owned());
                }
                if let Some(id) = caps.name("id") {
                    map.insert("id".to_owned(), id.as_str().to_owned());
                }
                map.insert("link".to_owned(), href_value.to_owned());
                map.insert("origin".to_owned(), origin.to_owned());
            }
        }
    }
    let feed_element = FeedElement::try_from(map);
}
