use block_mesh_common::constants::{
    DeviceType, BLOCKMESH_LOG_ENV, BLOCKMESH_VERSION, BLOCK_MESH_LOGGER,
};
use reqwest::Client;
#[cfg(feature = "sentry")]
use sentry_tracing;
use serde_json::{json, Value};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Once};
use std::thread;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{Event, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::layer::Context;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use uuid::Uuid;

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
            .with(tracing_subscriber::fmt::layer().with_ansi(false))
            .with(log_layer);

        #[cfg(feature = "sentry")]
        {
            sub.with(sentry_tracing::layer()).init();
        }
        #[cfg(not(feature = "sentry"))]
        {
            sub.init();
        }
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
        let init_client = Client::new();
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
    S: Subscriber,
    Self: 'static,
{
    fn on_event(&self, event: &Event, _ctx: Context<S>) {
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
