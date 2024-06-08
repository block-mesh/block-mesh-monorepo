#![allow(unused_macros, unused_imports)]

/// Logs output into browser console. It is not the same console as for the web page because the extension runs separately.
/// Look for the service worker console.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

macro_rules! log_info {
    ( $( $t:tt )* ) => {
        web_sys::console::info_1(&format!( $( $t )* ).into())
    }
}

macro_rules! log_warn {
    ( $( $t:tt )* ) => {
        web_sys::console::warn_1(&format!( $( $t )* ).into())
    }
}

macro_rules! log_error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
    }
}

macro_rules! log_debug {
    ( $( $t:tt )* ) => {
        web_sys::console::debug_1(&format!( $( $t )* ).into())
    }
}

pub(crate) use log;
pub(crate) use log_debug;
pub(crate) use log_error;
pub(crate) use log_info;
pub(crate) use log_warn;
