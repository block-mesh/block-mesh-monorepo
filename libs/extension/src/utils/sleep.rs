use js_sys::Promise;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn sleep(duration: Duration) {
    let _ = JsFuture::from(Promise::new(&mut |yes, _| {
        let w = window();
        if let Some(w) = w {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            );
        }
    }))
    .await;
}
