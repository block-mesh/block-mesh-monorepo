use database_utils::utils::connection::get_pg_pool;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgListener;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing_stdout_only_with_sentry();

    let pg_pool = get_pg_pool(None).await;
    let listen_task = tokio::spawn(listen(pg_pool.clone()));
    let notify_loop_task = tokio::spawn(notify_loop(pg_pool.clone()));

    tokio::select! {
        o = listen_task => panic!("listen_task exit {:?}", o),
        o = notify_loop_task => panic!("notify_loop_task exit {:?}", o)
    }
}

pub async fn listen(pool: PgPool) -> anyhow::Result<()> {
    let channels: Vec<&str> = vec!["test_channel"];
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(channels).await?;
    loop {
        while let Ok(Some(notification)) = listener.try_recv().await {
            let string = notification.payload().to_owned();
            if let Ok(_payload) = serde_json::from_str::<Value>(&string) {
            } else {
                tracing::error!("Failed to deserialize {:?}", string);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: Uuid,
    pub value: Value,
}

pub async fn notify(message: &Message, pool: &PgPool) -> anyhow::Result<()> {
    let s = serde_json::to_string(&message)?.replace('\'', "\"");
    let q = format!("NOTIFY test_channel , '{s}'");
    sqlx::query(&q).execute(pool).await?;
    Ok(())
}

pub async fn notify_loop(pool: PgPool) -> anyhow::Result<()> {
    let mut count = 0;
    loop {
        let msg: Message = Message {
            id: Uuid::new_v4(),
            value: Value::Null,
        };
        notify(&msg, &pool).await?;
        count += 1;
        if count % 100 == 0 {
            tracing::info!("loop count = {}", count);
        }
        if count == 50_000 {
            return Ok(());
        }
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
}
