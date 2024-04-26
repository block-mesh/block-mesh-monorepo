use crate::utils::connectors::{get_storage_value, set_storage_value};
use crate::utils::log::log;
use crate::utils::storage::StorageValues;
use anyhow::anyhow;
use leptos::{create_rw_signal, RwSignal, SignalGetUntracked};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub logged_in: RwSignal<bool>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("email", &self.email.get_untracked())
            .field("api_token", &self.api_token.get_untracked())
            .field("blockmesh_url", &self.blockmesh_url.get_untracked())
            .field("logged_in", &self.logged_in.get_untracked())
            .finish()
    }
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let blockmesh_url = Self::get_blockmesh_url().await;
        let email = Self::get_email().await;
        if email.is_empty() {
            log!("email is empty");
            return Err(anyhow!("email is empty"));
        }
        let api_token = Self::get_api_token().await;
        let api_token = match uuid::Uuid::from_str(&api_token) {
            Ok(v) => v,
            Err(e) => {
                log!("{e}");
                return Err(anyhow!("invalid api-token format"));
            }
        };
        Ok(Self {
            email: create_rw_signal(email),
            api_token: create_rw_signal(api_token),
            blockmesh_url: create_rw_signal(blockmesh_url),
            logged_in: create_rw_signal(false),
        })
    }

    pub async fn _store_blockmesh_url(blockmesh_url: String) {
        set_storage_value(
            &StorageValues::BlockMeshUrl.to_string(),
            JsValue::from_str(&blockmesh_url),
        )
        .await;
    }

    pub async fn _store_email(email: String) {
        set_storage_value(&StorageValues::Email.to_string(), JsValue::from_str(&email)).await;
    }

    pub async fn store_api_token(api_token: Uuid) {
        set_storage_value(
            &StorageValues::ApiToken.to_string(),
            JsValue::from_str(&api_token.to_string()),
        )
        .await;
    }

    pub async fn get_blockmesh_url() -> String {
        get_storage_value(StorageValues::BlockMeshUrl.to_string().as_str())
            .await
            .as_string()
            .unwrap_or("https://app.blockmesh.xyz".to_string())
    }

    pub async fn get_email() -> String {
        get_storage_value(StorageValues::Email.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }

    pub async fn get_api_token() -> String {
        get_storage_value(StorageValues::ApiToken.to_string().as_str())
            .await
            .as_string()
            .unwrap_or_default()
    }
}
