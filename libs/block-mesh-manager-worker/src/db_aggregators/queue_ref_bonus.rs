use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tracing::instrument(name = "queue_ref_bonus", skip_all)]
pub async fn queue_ref_bonus(
    mut rx: Receiver<Value>,
    queue: Arc<RwLock<HashSet<(Uuid, Uuid, NaiveDate)>>>,
) -> Result<(), anyhow::Error> {
    let enable = env::var("REF_BONUS_CRON_ENABLE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(DBMessage::DailyStatRefBonus(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
                    if enable {
                        let key = (message.user_id, message.daily_stat_id, message.day);
                        queue.write().await.insert(key);
                    }
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
