use flume::Sender;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tracing::error;

#[allow(dead_code)]
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
        while let Some(notification) = listener.try_recv().await? {
            let tx = tx.clone();
            tracing::info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let string = notification.payload().to_owned();
            if let Ok(payload) = serde_json::from_str::<T>(&string) {
                tracing::info!("des payload is {:?}", payload);
                call_back(payload, tx).await;
            } else {
                error!("Failed to deserialize {:?}", string);
            }
        }
    }
}
