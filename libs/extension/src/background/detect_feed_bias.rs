use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use anyhow::anyhow;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::DigestDataRequest;
use block_mesh_common::reqwest::http_client;
use leptos::logging::log;
use solana_sdk::signature::{Keypair, Signer};
use std::str::FromStr;
use std::string::ToString;
use twitter_scraping_helper::feed_element_try_from;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
const EXT_KEYPAIR: &str = env!("EXT_KEYPAIR");

pub fn get_keypair() -> anyhow::Result<Keypair> {
    let data: serde_json::Value =
        serde_json::from_str(EXT_KEYPAIR).map_err(|e| anyhow!(e.to_string()))?;
    let key_bytes: Vec<u8> = serde_json::from_value(data).map_err(|e| anyhow!(e.to_string()))?;
    Keypair::from_bytes(&key_bytes).map_err(|e| anyhow!(e.to_string()))
}

#[wasm_bindgen]
pub async fn feed_setup() {
    log!("Running feed_setup");
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

    let keypair = match get_keypair() {
        Ok(k) => k,
        Err(e) => {
            log!("Can't load keypair {}", e);
            return;
        }
    };

    match feed_element_try_from(&html, &origin) {
        Ok(feed_element) => {
            let client = http_client(DeviceType::Extension);
            let msg = format!("{}_{}", feed_element.user_name, feed_element.id);
            let signature = keypair.sign_message(msg.as_bytes()).to_string();
            let pubkey = keypair.pubkey().to_string();

            let body: DigestDataRequest = DigestDataRequest {
                email,
                api_token,
                data: feed_element,
                pubkey: Some(pubkey),
                msg: Some(msg),
                signature: Some(signature),
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
