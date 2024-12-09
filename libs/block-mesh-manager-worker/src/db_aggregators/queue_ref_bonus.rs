use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::NaiveDate;
use flume::Sender;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use uuid::Uuid;

#[tracing::instrument(name = "queue_ref_bonus", level = "trace", skip_all)]
pub async fn queue_ref_bonus(
    mut rx: Receiver<Value>,
    queue: Arc<RwLock<HashSet<(Uuid, Uuid, NaiveDate)>>>,
) -> Result<(), anyhow::Error> {
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(DBMessage::DailyStatRefBonus(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
                    let key = (message.user_id, message.daily_stat_id, message.day);
                    queue.write().await.insert(key);
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("queue_ref_bonus error recv: {:?}", e);
                    return Err(anyhow!("queue_ref_bonus error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("queue_ref_bonus error recv: {:?}", e);
                }
            },
        }
    }
}
