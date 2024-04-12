#[macro_use]
mod client;
mod api_wrappers;
mod constants;
mod models;

use constants::log;
use wasm_bindgen::prelude::*;

use web_sys::{Window, WorkerGlobalScope};

/// Contains the right type of the browser runtime for the current browser
pub(crate) enum BrowserRuntime {
    ChromeWorker(WorkerGlobalScope),
    FireFoxWindow(Window),
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Makes JS `console.log` available in Rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

/// A demo function to test if WASM is callable from background.js
#[wasm_bindgen]
pub fn hello_wasm() {
    log("Hello from WASM!");
}

/// The main entry point callable from `background.js`.
#[wasm_bindgen]
pub async fn add_random_tracks(
    auth_header_value: &str,
    token_header_value: &str,
    playlist_id: &str,
    user_uri: &str,
    number_of_tracks_to_add: Option<u32>,
) {
    // try to init the browser runtime, but there is nothing we can do if it's missing
    // if it does, there is either a bug or something changed in the browser implementation
    // The runtime is a global singleton. It can probably work with OnceCell or lazy_static!.
    let runtime = match get_runtime().await {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            report_progress(e);
            return;
        }
    };

    // the number of tracks comes from a dropdown and may potentially be invalid
    let number_of_tracks_to_add: usize = match number_of_tracks_to_add {
        Some(v) => match v.try_into() {
            Ok(v) => v,
            Err(e) => {
                report_progress(&format!(
                    "Cannot convert number_of_tracks_to_add. It's a bug. {number_of_tracks_to_add:?}, {e:?}"
                ));
                return;
            }
        },
        None => constants::DEFAULT_PLAYLIST_SIZE,
    };

    // log the result for debugging and send and copy
    // of the same message to whatever frontend is listening
    // via JS sendMessage
    match client::generate_random_playlist(
        auth_header_value,
        token_header_value,
        playlist_id,
        user_uri,
        number_of_tracks_to_add,
        &runtime,
    )
    .await
    {
        Ok(v) => {
            log!("{v}");
            report_progress(&v);
        }

        Err(v) => {
            log!("{v}");
            report_progress(&v);
        }
    };
}

/// This is a proxy for report_progress() in progress.js
/// to send messages to other js scripts.
#[wasm_bindgen(module = "/src/progress.js")]
extern "C" {
    pub fn report_progress(msg: &str);
}

/// All error handling in this crate is based on either retrying a request after some time
/// or exiting gracefully.
#[derive(Debug, Clone)]
pub enum RetryAfter {
    Seconds(i64),
    Never,
}

/// The result type that should be used in place of std::Result
/// throughout the app
pub type Result<T> = std::result::Result<T, RetryAfter>;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// pub async fn sleep(duration: Duration) {
//     JsFuture::from(Promise::new(&mut |yes, _| {
//         window()
//             .unwrap()
//             .set_timeout_with_callback_and_timeout_and_arguments_0(
//                 &yes,
//                 duration.as_millis() as i32,
//             )
//             .unwrap();
//     }))
//     .await
//     .unwrap();
// }

/// Returns the right type of runtime for the current browser because
/// Firefox and Chrome do not agree on the parent object for Runtime in WebWorkers.
/// Firefox uses Window and Chrome uses WorkerGlobalScope.
async fn get_runtime() -> std::result::Result<BrowserRuntime, &'static str> {
    // try for chrome first and return if found
    // it should also work if FF switches to using WorkerGlobalScope as they should
    match js_sys::global().dyn_into::<WorkerGlobalScope>() {
        Ok(v) => {
            return Ok(BrowserRuntime::ChromeWorker(v));
        }
        Err(e) => {
            log!("ServiceWorkerGlobalScope unavailable");
            log!("{:?}", e);
        }
    };

    // this is a fallback for Firefox, but it does not make sense why they would use Window in
    // web workers
    match web_sys::window() {
        Some(v) => {
            return Ok(BrowserRuntime::FireFoxWindow(v));
        }
        None => {
            log!("Window unavailable");
        }
    };

    // no runtime was found, which is a serious problem
    // because all fetch calls require it
    // TODO: may be worth a retry
    Err("Missing browser runtime. It's a bug.")
}
