use crate::login_mode::login_mode;
use chrono::Utc;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;

use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use reqwest::Client;
use tokio::time::sleep;

#[no_mangle]
#[used]
pub static mut RUNNING: i8 = 0;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn Java_xyz_blockmesh_runLib(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
    email: JString,
    password: JString,
) -> jint {
    let runtime = Arc::new(
        Builder::new_multi_thread()
            .thread_name("blockmesh-cli")
            .enable_all()
            .build()
            .unwrap(),
    );
    let url: String = match env.get_string(&url) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Failed to load url {}", e);
            return -1;
        }
    };

    let email: String = match env.get_string(&email) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Failed to load email {}", e);
            return -1;
        }
    };

    let password: String = match env.get_string(&password) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Failed to load password {}", e);
            return -1;
        }
    };

    runtime.block_on(async {
        let _ = login_mode(&url, &email, &password).await;
    });
    -1
}

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn stop_lib() -> i8 {
    // RUNNING = -2;
    let runtime = Arc::new(
        Builder::new_multi_thread()
            .thread_name("blockmesh-cli-stop-lib")
            .enable_all()
            .build()
            .unwrap(),
    );
    runtime.block_on(async {
        let _ = Client::new()
            .get(format!(
                "{}/health_check?RUNNING={}&stop_lib=before",
                "http://localhost:8000", RUNNING
            ))
            .send()
            .await;
        // RUNNING = -1;
        let _ = Client::new()
            .get(format!(
                "{}/health_check?RUNNING={}&stop_lib=after",
                "http://localhost:8000", RUNNING
            ))
            .send()
            .await;
    });
    RUNNING
}

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn run_lib(
    url: *const c_char,
    email: *const c_char,
    password: *const c_char,
) -> i8 {
    if RUNNING == 1 {
        return 0;
    }
    let url = match unsafe { CStr::from_ptr(url) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load url {}", e);
            return -1;
        }
    };
    let email = match unsafe { CStr::from_ptr(email) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load email {}", e);
            return -1;
        }
    };
    let password = match unsafe { CStr::from_ptr(password) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load password {}", e);
            return -1;
        }
    };

    let runtime = Arc::new(
        Builder::new_multi_thread()
            .thread_name("blockmesh-cli-run-lib")
            .enable_all()
            .build()
            .unwrap(),
    );

    runtime.block_on(async {
        RUNNING = 1;
        loop {
            if RUNNING == -1 {
                break;
            }
            let now = Utc::now();
            let _ = Client::new()
                .get(format!(
                    "{}/health_check?time={}&RUNNING={}",
                    url, now, RUNNING
                ))
                .send()
                .await;
            sleep(Duration::from_secs(5)).await
        }
        // let _ = login_mode(url, email, password).await;
    });
    RUNNING = 0;
    -1
}
