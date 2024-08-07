use crate::login_mode::login_mode;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use tokio::runtime::Builder;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn run_lib(
    url: *const c_char,
    email: *const c_char,
    password: *const c_char,
) -> i8 {
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
            .thread_name("blockmesh-cli")
            .enable_all()
            .build()
            .unwrap(),
    );

    runtime.block_on(async {
        let _ = login_mode(url, email, password).await;
    });
    -1
}
