use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{DigestDataRequest, FeedElement};
use block_mesh_common::reqwest::http_client;
use leptos::logging::log;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn feed_setup() {
    ExtensionWrapperState::store_feed_origin(env!("FEED_ORIGIN").to_string()).await;
    ExtensionWrapperState::store_feed_selector(env!("FEED_SELECTOR").to_string()).await;
}

#[wasm_bindgen]
pub async fn read_dom(html: String, origin: String) {
    let mut blockmesh_data_sink_url = ExtensionWrapperState::get_blockmesh_data_sink_url().await;
    if blockmesh_data_sink_url.is_empty() {
        blockmesh_data_sink_url = "https://data-sink.blockmesh.xyz".to_string();
        ExtensionWrapperState::store_blockmesh_data_sink_url(blockmesh_data_sink_url.clone()).await;
    }
    let email = ExtensionWrapperState::get_email().await;
    let api_token = ExtensionWrapperState::get_api_token().await;
    let api_token = uuid::Uuid::from_str(&api_token).unwrap_or_else(|_| Uuid::default());
    if blockmesh_data_sink_url.is_empty()
        || email.is_empty()
        || api_token == Uuid::default()
        || api_token.is_nil()
    {
        log!(
            "early return from read_dom => url = {} , email = {} , api_token = {}",
            blockmesh_data_sink_url,
            email,
            api_token
        );
        return;
    }

    let fragment = Html::parse_fragment(&html);
    let href = Selector::parse("[href]").unwrap();
    let re = Regex::new(r"/(?P<username>[^/]+)/status/(?P<id>\d+$)").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("raw".to_owned(), html);
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
    match FeedElement::try_from(map) {
        Ok(feed_element) => {
            let client = http_client(DeviceType::Extension);
            let body: DigestDataRequest = DigestDataRequest {
                email,
                api_token,
                data: feed_element,
            };
            let _ = client
                .post(format!("{}/digest_data", blockmesh_data_sink_url))
                .json(&body)
                .send()
                .await;
        }
        Err(e) => {
            log!("error = {:?}", e);
        }
    }
}
