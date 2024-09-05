use serde::Deserialize;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use tracing::error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Payload(pub Value);

#[allow(dead_code)]
pub async fn start_listening<T: DeserializeOwned + Sized + Debug>(
    pool: Pool<Postgres>,
    channels: Vec<&str>,
    call_back: impl Fn(T),
) -> Result<(), Error> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(channels).await?;
    loop {
        while let Some(notification) = listener.try_recv().await? {
            tracing::info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let string = notification.payload().to_owned();
            if let Ok(payload) = serde_json::from_str::<T>(&string) {
                tracing::info!("des payload is {:?}", payload);
                call_back(payload);
            } else {
                error!("Failed to deserialize {:?}", string);
            }
        }
    }
}
