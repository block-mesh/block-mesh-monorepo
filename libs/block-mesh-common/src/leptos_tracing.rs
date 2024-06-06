use crate::constants::{DeviceType, BLOCKMESH_VERSION, BLOCK_MESH_LOGGER, BLOCK_MESH_LOG_ENV};
use reqwest::Client;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::{Arc, Once};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::MakeWriter;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

static UUID_CELL: OnceLock<Uuid> = OnceLock::new();

#[inline]
pub fn setup_leptos_tracing(user_id: Option<Uuid>, device_type: DeviceType) {
    static SET_HOOK: Once = Once::new();
    if let Some(user_id) = user_id {
        if UUID_CELL.get().is_none() {
            UUID_CELL.set(user_id).unwrap();
        }
    }
    SET_HOOK.call_once(|| {
        let buffer: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
        let client: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new()));
        let url = Arc::new(BLOCK_MESH_LOGGER.to_string());
        let env = std::env::var(BLOCK_MESH_LOG_ENV).unwrap_or_else(|_| "prod".to_string());

        let writer = HttpMakeConsoleWriter {
            client,
            buffer,
            url,
            env,
            device_type,
            mapped_levels: MappedLevels::default(),
        };
        fmt()
            .with_writer(writer.map_trace_level_to(tracing::Level::DEBUG))
            .with_ansi(false)
            .without_time()
            .init();
    });
}

#[derive(Clone, Debug)]
pub struct HttpMakeConsoleWriter {
    pub client: Arc<Mutex<Client>>,
    pub buffer: Arc<Mutex<Vec<Value>>>,
    pub url: Arc<String>,
    pub env: String,
    pub device_type: DeviceType,
    pub mapped_levels: MappedLevels,
}

impl HttpMakeConsoleWriter {
    /// Maps the [`tracing::Level::TRACE`] to another console level.
    pub fn map_trace_level_to(mut self, level: tracing::Level) -> Self {
        self.mapped_levels.trace = level;

        self
    }

    /// Maps the [`tracing::Level::DEBUG`] to another console level.
    pub fn map_debug_level_to(mut self, level: tracing::Level) -> Self {
        self.mapped_levels.debug = level;

        self
    }

    /// Maps the [`tracing::Level::INFO`] to another console level.
    pub fn map_info_level_to(mut self, level: tracing::Level) -> Self {
        self.mapped_levels.info = level;

        self
    }

    /// Maps the [`tracing::Level::WARN`] to another console level.
    pub fn map_warn_level_to(mut self, level: tracing::Level) -> Self {
        self.mapped_levels.warn = level;

        self
    }

    /// Maps the [`tracing::Level::ERROR`] to another console level.
    pub fn map_error_level_to(mut self, level: tracing::Level) -> Self {
        self.mapped_levels.error = level;
        self
    }
}

impl<'a> MakeWriter<'a> for HttpMakeConsoleWriter {
    type Writer = ConsoleWriter;

    fn make_writer(&'a self) -> Self::Writer {
        unimplemented!("use make_writer_for instead");
    }

    fn make_writer_for(&'a self, meta: &tracing::Metadata<'_>) -> Self::Writer {
        ConsoleWriter {
            url: self.url.clone(),
            level: *meta.level(),
            data: Vec::with_capacity(256),
            device_type: self.device_type,
            env: self.env.clone(),
            client: self.client.clone(),
        }
    }
}

/// Allows mapping [`tracing::Level`] events to a different
/// console level.
#[derive(Clone, Copy, Debug)]
pub struct MappedLevels {
    /// The verbosity level [`tracing::Level::TRACE`] events should be mapped to
    /// in the console.
    pub trace: tracing::Level,
    /// The verbosity level [`tracing::Level::DEBUG`] events should be mapped to
    /// in the console.
    pub debug: tracing::Level,
    /// The verbosity level [`tracing::Level::INFO`] events should be mapped to
    /// in the console.
    pub info: tracing::Level,
    /// The verbosity level [`tracing::Level::WARN`] events should be mapped to
    /// in the console.
    pub warn: tracing::Level,
    /// The verbosity level [`tracing::Level::ERROR`] events should be mapped to
    /// in the console.
    pub error: tracing::Level,
}

impl Default for MappedLevels {
    fn default() -> Self {
        Self {
            trace: tracing::Level::TRACE,
            debug: tracing::Level::DEBUG,
            info: tracing::Level::INFO,
            warn: tracing::Level::WARN,
            error: tracing::Level::ERROR,
        }
    }
}

/// The type which is responsible for actually writing the tracing
/// event out to the console.
pub struct ConsoleWriter {
    pub level: tracing::Level,
    pub data: Vec<u8>,
    pub device_type: DeviceType,
    pub env: String,
    pub client: Arc<Mutex<Client>>,
    pub url: Arc<String>,
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

impl Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        use tracing::Level;
        let data = String::from_utf8(self.data.clone()).unwrap_or_default();
        let json = json!({
            "level": self.level.to_string(),
            "event": data,
            "device_type": self.device_type.clone(),
            "version": BLOCKMESH_VERSION,
            "user_id": match UUID_CELL.get() {
                Some(uuid) => uuid.to_string(),
                None => "".to_string(),
            },
        });

        match self.level {
            Level::TRACE => {
                log_debug!("{}", data)
            }
            Level::DEBUG => {
                log_debug!("{}", data)
            }
            Level::INFO => {
                log_info!("{}", data)
            }
            Level::WARN => {
                log_warn!("{}", data)
            }
            Level::ERROR => {
                log_error!("{}", data)
            }
        }

        let client = self.client.clone().lock().unwrap().clone();
        let url = self.url.clone();
        spawn_local(async move {
            let r = client.post(&*url).json(&json).send().await;
            match r {
                Ok(_) => {}
                Err(e) => log_error!("Error sending logs: {:?}", e),
            }
        });
        Ok(())
    }
}

impl Drop for ConsoleWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
