use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use block_mesh_manager_database_domain::domain::fetch_latest_cron_settings::fetch_latest_cron_settings;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_base_msg_loop", skip_all)]
pub async fn ws_base_msg_loop(
    pool: PgPool,
    server_user_id: Uuid,
    broadcaster: Broadcaster,
) -> anyhow::Result<()> {
    let messages = vec![
        WsServerMessage::RequestUptimeReport,
        WsServerMessage::RequestBandwidthReport,
    ];
    loop {
        let settings = match fetch_latest_cron_settings(&pool, &server_user_id).await {
            Ok(settings) => settings,
            Err(e) => {
                tracing::error!("fetch_latest_cron_settings error {}", e);
                tokio::time::sleep(Duration::from_millis(30_000)).await;
                continue;
            }
        };
        let new_period = settings.period;

        for i in broadcaster.sockets.iter() {
            let id = i.key().clone();
            broadcaster.broadcast_to_user(messages.clone(), id).await;
        }
        tokio::time::sleep(new_period).await;
    }
}
