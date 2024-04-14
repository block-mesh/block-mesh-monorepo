/// Logs output into browser console. It is not the same console as for the web page because the extension runs separately.
/// Look for the service worker console.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

pub(crate) use log;
