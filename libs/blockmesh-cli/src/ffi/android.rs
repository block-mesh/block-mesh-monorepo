use chrono::Utc;
use std::time::Duration;

use crate::ffi::ffi_common::*;
use crate::jstring_to_str;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use reqwest::Client;
use tokio::time::sleep;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn Java_expo_modules_myrustmodule_MyRustModule_runLib(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
    email: JString,
    password: JString,
) -> jint {
    // let runtime = create_current_thread_runtime();
    let url: String = jstring_to_str!(&url, env, "url");
    let _email: String = jstring_to_str!(&email, env, "email");
    let _password: String = jstring_to_str!(&password, env, "password");
    // runtime.block_on(async {
    //     let _ = login_mode(&url, &email, &password).await;
    // });
    // -1

    if get_status() != 0 {
        return 0;
    }

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
/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn Java_expo_modules_myrustmodule_MyRustModule_stopLib(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let runtime = create_current_thread_runtime();
    runtime.block_on(async {
        debug_stop(CLOUDFLARE).await;
        debug_stop(NGROK).await;
        debug_stop(LOCALHOST).await;
        debug_stop(LOCALHOST_2).await;
    });
    11
}
