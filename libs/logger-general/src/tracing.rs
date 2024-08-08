use block_mesh_common::constants::{
    DeviceType, BLOCKMESH_LOG_ENV, BLOCKMESH_VERSION, BLOCK_MESH_LOGGER,
};
use reqwest::{Client, ClientBuilder};
use serde_json::{json, Value};
use std::option::Option;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Once};
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::span::Attributes;
use tracing::{field, Event, Id, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::layer::Context;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use uuid::Uuid;

const FILTER_SPAM_MODULE_PATH: [&str; 3] = [
    "axum_login::service",
    "tower_sessions::service",
    "tower_sessions_core::session",
];

struct Timings {
    idle: u64,
    busy: u64,
    last: Instant,
}

impl Timings {
    fn new() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: Instant::now(),
        }
    }
}

macro_rules! with_event_from_span {
    ($id:ident, $span:ident, $($field:literal = $value:expr),*, |$event:ident| $code:block) => {
        let meta = $span.metadata();
        let cs = meta.callsite();
        let fs = field::FieldSet::new(&[$($field),*], cs);
        #[allow(unused)]
        let mut iter = fs.iter();
        let v = [$(
            (&iter.next().unwrap(), ::core::option::Option::Some(&$value as &dyn field::Value)),
        )*];
        let vs = fs.value_set(&v);
        let $event = Event::new_child_of($id, meta, &vs);
        $code
    };
}

pub fn setup_tracing_stdout_only() {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        let sub = tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer().with_ansi(false));
        sub.init();
    });
}

pub fn setup_tracing(user_id: Uuid, device_type: DeviceType) {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        let log_env = std::env::var(BLOCKMESH_LOG_ENV).unwrap_or_else(|_| "prod".to_string());
        let log_layer =
            HttpLogLayer::new(BLOCK_MESH_LOGGER.to_string(), log_env, user_id, device_type);
        let sub = tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(
                tracing_subscriber::fmt::layer().with_ansi(false), // .with_span_events(FmtSpan::CLOSE),
            )
            .with(log_layer);
        sub.init();
    });
}

struct HttpLogLayer {
    pub client: Client,
    pub buffer: Arc<Mutex<Vec<Value>>>,
    pub url: Arc<String>,
    pub env: String,
    pub user_id: Arc<Uuid>,
    pub device_type: DeviceType,
    pub tx: Sender<JoinHandle<()>>,
}

impl HttpLogLayer {
    fn new(url: String, env: String, user_id: Uuid, device_type: DeviceType) -> Self {
        let init_buffer: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
        let init_client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let user_id = Arc::new(user_id);
        let init_url = Arc::new(url);
        let x_url = init_url.clone();
        let x_buffer = init_buffer.clone();
        let x_client = init_client.clone();
        let (tx, rx): (Sender<JoinHandle<()>>, Receiver<JoinHandle<()>>) = mpsc::channel();

        thread::spawn(move || async move {
            while let Ok(handle) = rx.recv() {
                let _ = handle.await;
            }
        });

        tokio::spawn(async move {
            let client = init_client.clone();
            let url = init_url.clone();
            let buffer = init_buffer.clone();
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                let logs = {
                    let mut buffer = buffer.lock().await;
                    std::mem::take(&mut *buffer)
                };
                if !logs.is_empty() {
                    HttpLogLayer::send_logs(client.clone(), url.clone(), logs).await;
                }
            }
        });

        Self {
            tx,
            client: x_client.clone(),
            buffer: x_buffer.clone(),
            url: x_url.clone(),
            env,
            user_id,
            device_type,
        }
    }

    async fn send_logs(client: Client, url: Arc<String>, logs: Vec<Value>) {
        let r = client.post(&*url).json(&logs).send().await;
        match r {
            Ok(_) => {}
            Err(e) => println!("Error sending logs: {:?}", e),
        }
    }
}

impl<S> Layer<S> for HttpLogLayer
where
    S: Subscriber
        + for<'span> tracing_subscriber::registry::LookupSpan<'span>
        + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
    Self: 'static,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if extensions.get_mut::<Timings>().is_none() {
            extensions.insert(Timings::new());
        }

        with_event_from_span!(id, span, "message" = "new", |event| {
            drop(extensions);
            drop(span);
            self.on_event(&event, ctx);
        });
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        let mut busy: Option<u64> = None;
        let mut idle: Option<u64> = None;
        if let Some(timings) = extensions.get_mut::<Timings>() {
            let now = Instant::now();
            timings.busy += (now - timings.last).as_nanos() as u64;
            timings.last = now;
            busy = Option::from(timings.busy);
            idle = Option::from(timings.idle);
        }
        with_event_from_span!(
            id,
            span,
            "message" = "exit",
            "time.busy" = busy,
            "time.idle" = idle,
            |event| {
                drop(extensions);
                drop(span);
                self.on_event(&event, ctx);
            }
        );
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        let mut busy: Option<u64> = None;
        let mut idle: Option<u64> = None;

        if let Some(timings) = extensions.get_mut::<Timings>() {
            let now = Instant::now();
            timings.idle += (now - timings.last).as_nanos() as u64;
            timings.last = now;
            busy = Option::from(timings.busy);
            idle = Option::from(timings.idle);
        }
        with_event_from_span!(
            id,
            span,
            "message" = "enter",
            "time.busy" = busy,
            "time.idle" = idle,
            |event| {
                drop(extensions);
                drop(span);
                self.on_event(&event, ctx);
            }
        );
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        if let Some(timing) = extensions.get::<Timings>() {
            let Timings {
                busy,
                mut idle,
                last,
            } = *timing;
            idle += (Instant::now() - last).as_nanos() as u64;

            with_event_from_span!(
                id,
                span,
                "message" = "close",
                "time.busy" = busy,
                "time.idle" = idle,
                |event| {
                    drop(extensions);
                    drop(span);
                    self.on_event(&event, ctx);
                }
            );
        } else {
            with_event_from_span!(id, span, "message" = "close", |event| {
                drop(extensions);
                drop(span);
                self.on_event(&event, ctx);
            });
        }
    }

    fn on_event(&self, event: &Event, _ctx: Context<S>) {
        if let Some(module_path) = event.metadata().module_path() {
            if FILTER_SPAM_MODULE_PATH.contains(&module_path) {
                return;
            }
        }
        let user_id = self.user_id.clone();
        let log = json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "level": event.metadata().level().to_string(),
            "event": event.as_serde(),
            "env": self.env.clone(),
            "user_id": *user_id,
            "device_type": self.device_type.clone(),
            "version": BLOCKMESH_VERSION,
        });

        let buffer = self.buffer.clone();
        let url = self.url.clone();
        let client = self.client.clone();
        let handle = tokio::spawn(async move {
            let mut buffer = buffer.lock().await;
            buffer.push(log);
            if buffer.len() >= 10 {
                let logs = { std::mem::take(&mut *buffer) };
                drop(buffer); // release the lock before sending logs
                HttpLogLayer::send_logs(client, url, logs).await;
            }
        });
        let _ = self.tx.send(handle);
    }
}
