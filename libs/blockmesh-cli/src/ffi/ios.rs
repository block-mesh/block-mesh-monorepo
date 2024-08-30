use crate::char_to_str;
use crate::ffi::ffi_common::*;
use std::os::raw::c_char;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn run_lib(
    url: *const c_char,
    email: *const c_char,
    password: *const c_char,
) -> i8 {
    if get_status() != FFIStatus::WAITING {
        return 0;
    }
    let url = char_to_str!(url, "url");
    let _email = char_to_str!(email, "email");
    let _password = char_to_str!(password, "password");
    debug_running(url);
    -1
}

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn stop_lib(url: *const c_char) -> i8 {
    let url = char_to_str!(url, "url");
    debug_stop(url);
    0
}

/// # Safety
/// This method give insight into current status of lib
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn get_lib_status() -> i8 {
    get_status().into()
}
