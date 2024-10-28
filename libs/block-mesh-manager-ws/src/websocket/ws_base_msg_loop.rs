use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use block_mesh_manager_database_domain::domain::fetch_latest_cron_settings::fetch_latest_cron_settings;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_base_msg_loop", skip_all)]
pub async fn ws_base_msg_loop(
    pool: PgPool,
    server_user_id: Uuid,
    broadcaster: Broadcaster,
) -> anyhow::Result<()> {
    let queue_size = env::var("QUEUE_SIZE")
        .unwrap_or("100".to_string())
        .parse()?;
    let in_between_iterations = Duration::from_millis(
        env::var("IN_BETWEEN_ITERATIONS_TIME")
            .unwrap_or("100".to_string())
            .parse()?,
    );
    let messages = vec![
        WsServerMessage::RequestUptimeReport,
        WsServerMessage::RequestBandwidthReport,
    ];
    let base_msg_sleep = Duration::from_millis(
        env::var("BASE_MSG_SLEEP")
            .unwrap_or("300000".to_string())
            .parse()?,
    );

    loop {
        let iterations = broadcaster.sockets.len() / queue_size + 1;
        for _ in 0..iterations {
            broadcaster
                .queue_multiple(messages.clone(), queue_size)
                .await;
            tokio::time::sleep(in_between_iterations).await;
        }
        tokio::time::sleep(base_msg_sleep).await;
    }
}
