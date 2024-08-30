use crate::ffi::ffi_common::*;
use crate::jstring_to_str;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;

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
    let url: String = jstring_to_str!(&url, env, "url");
    let _email: String = jstring_to_str!(&email, env, "email");
    let _password: String = jstring_to_str!(&password, env, "password");
    debug_running(&url);
    -1
}

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn Java_expo_modules_myrustmodule_MyRustModule_stopLib(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jint {
    let url: String = jstring_to_str!(&url, env, "url");
    debug_stop(&url);
    0
}

/// # Safety
/// This method give insight into current status of lib
/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn Java_expo_modules_myrustmodule_MyRustModule_getLibStatus(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let i: i8 = get_status().into();
    i as jint
}
