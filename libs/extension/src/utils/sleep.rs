use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(dead_code)]
pub async fn sleep(millis: u32) {
    TimeoutFuture::new(millis).await;
}
