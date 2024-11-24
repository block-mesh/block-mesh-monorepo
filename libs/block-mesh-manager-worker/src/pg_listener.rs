use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tracing::error;

#[tracing::instrument(name = "start_listening", skip_all, err)]
pub async fn start_listening<T, F, R, Fut>(
    pool: Pool<Postgres>,
    channels: Vec<&str>,
    tx: Sender<Value>,
    call_back: F,
) -> Result<(), Error>
where
    T: DeserializeOwned + Sized + Debug,
    F: Fn(T, Arc<Sender<Value>>) -> Fut,
    Fut: Future<Output = R>,
{
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(channels).await?;
    let tx = Arc::new(tx);
    loop {
        while let Ok(Some(notification)) = listener.try_recv().await {
            let tx = tx.clone();
            let string = notification.payload().to_owned();
            if let Ok(payload) = serde_json::from_str::<T>(&string) {
                call_back(payload, tx).await;
            } else {
                error!("Failed to deserialize {:?}", string);
            }
        }
    }
}
