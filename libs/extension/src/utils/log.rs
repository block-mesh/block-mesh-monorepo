use leptos::tracing;
// use leptos::tracing::level_filters::LevelFilter;
use std::sync::Once;
// use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
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

#[inline]
pub fn setup_leptos_tracing() {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let r = Registry::default().with(env_filter);
        // let subscriber = Subscriber::builder().without_time().finish();
        // Initialize the tracing subscriber
        tracing::subscriber::set_global_default(r).expect("setting tracing default failed");
    });
}

pub(crate) use log;
pub(crate) use log_error;
pub(crate) use log_info;
pub(crate) use log_warn;
