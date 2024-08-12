use crate::char_to_str;
use crate::ffi::ffi_common::*;
use chrono::Utc;
use reqwest::Client;
use std::os::raw::c_char;
use std::time::Duration;
use tokio::time::sleep;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn run_lib(
    url: *const c_char,
    email: *const c_char,
    password: *const c_char,
) -> i8 {
    if get_status() != 0 {
        return 0;
    }
    let url = char_to_str!(url, "url");
    let _email = char_to_str!(email, "email");
    let _password = char_to_str!(password, "password");

    let runtime = create_current_thread_runtime();
    runtime.block_on(async {
        set_status(1);
        loop {
            if get_status() == -1 {
                break;
            }
            let v = get_status();
            set_status(v + 1);
            let now = Utc::now();
            let _ = Client::new()
                .get(format!(
                    "{}/health_check?time={}&RUNNING={}",
                    url,
                    now,
                    get_status()
                ))
                .send()
                .await;
            sleep(Duration::from_secs(5)).await
        }
        // let _ = login_mode(url, email, password).await;
    });
    set_status(0);
    -1
}

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn stop_lib() -> i8 {
    let runtime = create_current_thread_runtime();
    runtime.block_on(async {
        debug_stop(CLOUDFLARE).await;
        debug_stop(NGROK).await;
        debug_stop(LOCALHOST).await;
        debug_stop(LOCALHOST_2).await;
    });
    get_status()
}
