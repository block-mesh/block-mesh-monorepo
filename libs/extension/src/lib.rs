use crate::tasks::{get_task, run_task, submit_task};
use crate::utils::connectors::{get_storage_value, set_panic_hook};
use crate::utils::log::log;
use std::fmt::Display;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

mod tasks;
mod utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub enum StorageValues {
    BlockMeshUrl,
    Email,
    ApiToken,
}

impl Display for StorageValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StorageValues::BlockMeshUrl => "blockmesh_url".to_string(),
            StorageValues::Email => "email".to_string(),
            StorageValues::ApiToken => "api_token".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// The main entry point callable from `background.js`.
#[wasm_bindgen]
pub async fn task_poller() {
    set_panic_hook();
    // try to init the browser runtime, but there is nothing we can do if it's missing
    // if it does, there is either a bug or something changed in the browser implementation
    // The runtime is a global singleton. It can probably work with OnceCell or lazy_static!.
    // match get_runtime().await {
    //     Ok(v) => v,
    //     Err(e) => {
    //         log!("{e}");
    //         return;
    //     }
    // };

    let blockmesh_url = get_storage_value(StorageValues::BlockMeshUrl.to_string().as_str())
        .await
        .as_string()
        .unwrap_or("https://app.blockmesh.xyz".to_string());

    let email = get_storage_value(StorageValues::Email.to_string().as_str())
        .await
        .as_string()
        .unwrap_or_default();
    let api_token = get_storage_value(StorageValues::ApiToken.to_string().as_str())
        .await
        .as_string()
        .unwrap_or_default();
    if email.is_empty() {
        log!("email is empty");
        return;
    }
    if api_token.is_empty() {
        log!("api_token is empty");
        return;
    }
    let api_token = match uuid::Uuid::from_str(&api_token) {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            return;
        }
    };
    log!("blockmesh_url => {:?}", blockmesh_url);

    let task = match get_task(&blockmesh_url, &email, &api_token).await {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            return;
        }
    };
    let finished_task = match run_task(&task.url, &task.method, task.headers, task.body).await {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            return;
        }
    };

    match submit_task(
        &blockmesh_url,
        &email,
        &api_token,
        &task.id,
        Option::from(finished_task.status),
        Option::from(finished_task.raw),
    )
    .await
    {
        Ok(_) => {
            log!("successfully submitted task");
        }
        Err(e) => {
            log!("{e}");
        }
    };
}
