use crate::state::AppState;
use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use block_mesh_manager_database_domain::domain::fetch_latest_cron_settings::fetch_latest_cron_settings;
use block_mesh_manager_database_domain::domain::find_users_tasks::find_users_tasks;
use block_mesh_manager_database_domain::domain::task::TaskStatus;
use block_mesh_manager_database_domain::domain::task_limit::TaskLimit;
use block_mesh_manager_database_domain::domain::update_task_assigned::update_task_assigned;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_task_loop", skip_all)]
pub async fn ws_task_loop(
    pool: PgPool,
    server_user_id: Uuid,
    broadcaster: Broadcaster,
    state: Arc<AppState>,
) -> anyhow::Result<()> {
    let mut redis = state.redis.clone();
    let task_limit = env::var("TASK_LIMIT").unwrap_or("10".to_string()).parse()?;
    let expire = 10u64
        * env::var("REDIS_EXPIRE")
            .unwrap_or("86400".to_string())
            .parse::<u64>()?;

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
        let new_window_size = settings.window_size;
        let mut queued = broadcaster.move_queue(new_window_size).await;
        let mut transaction = match create_txn(&pool).await {
            Ok(transaction) => transaction,
            Err(e) => {
                tracing::error!("create_txn error {}", e);
                tokio::time::sleep(Duration::from_millis(30_000)).await;
                continue;
            }
        };
        let mut tasks = match find_users_tasks(&mut transaction, new_window_size as i64).await {
            Ok(tasks) => tasks,
            Err(e) => {
                tracing::error!("find_users_tasks error {}", e);
                tokio::time::sleep(Duration::from_millis(30_000)).await;
                continue;
            }
        };
        loop {
            let task = match tasks.pop() {
                Some(t) => t,
                None => break,
            };
            let queue = match queued.pop() {
                Some(q) => q,
                None => break,
            };
            let user_id = queue.0;

            let mut user_limit =
                match TaskLimit::get_task_limit(&user_id, &mut redis, task_limit).await {
                    Ok(l) => l,
                    Err(e) => {
                        tracing::error!("ws_task_loop get_task_limit {} {}", user_id, e);
                        continue;
                    }
                };
            if user_limit.tasks > task_limit {
                continue;
            }

            let _ = broadcaster
                .broadcast_to_user(
                    vec![WsServerMessage::AssignTask(GetTaskResponse {
                        id: task.id,
                        url: task.url,
                        method: task.method.to_string(),
                        headers: task.headers,
                        body: task.body,
                    })],
                    queue,
                )
                .await;
            let _ = update_task_assigned(&mut transaction, task.id, user_id, TaskStatus::Assigned)
                .await;
            user_limit.tasks += 1;
            TaskLimit::save_user(&mut redis, &user_limit, expire).await;
        }
        match commit_txn(transaction).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("commit_txn {}", e);
            }
        }
        tokio::time::sleep(new_period).await;
    }
}
