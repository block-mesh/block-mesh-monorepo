use crate::ore::ore_process_wrapper::{kill_ore_process, ore_process_wrapper, OreProcessInput};
use crate::tauri_state::AppState;
use crate::TOKIO_JOINER_TX;
use block_mesh_common::app_config::TaskStatus;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tracing::instrument(name = "ore_process_monitor", skip(ore_rx, app_state), ret)]
pub async fn ore_process_monitor(
    mut ore_rx: tokio::sync::mpsc::Receiver<TaskStatus>,
    app_state: Arc<Mutex<AppState>>,
) -> Result<(), anyhow::Error> {
    while let Some(task_status) = ore_rx.recv().await {
        let mut state = app_state.lock().await;
        if let Some(ore_pid) = state.ore_pid {
            match kill_ore_process(ore_pid).await {
                Ok(_) => {
                    state.config.ore_status = Option::from(TaskStatus::Off);
                    state.ore_pid = None;
                }
                Err(e) => {
                    tracing::error!("Error killing child process: {:?}", e);
                }
            }
        }

        match task_status {
            TaskStatus::Off => {
                state.config.ore_status = Option::from(TaskStatus::Off);
            }
            TaskStatus::Running => {
                let exec = std::env::current_exe().unwrap_or_default();
                let ore = exec.parent().unwrap().join("resources").join("ore");
                if tokio::fs::metadata(&ore).await.is_err() {
                    log::error!("ORE binary not found at {:?}", ore);
                    state.config.ore_status = Option::from(TaskStatus::Off);
                    continue;
                }

                match ore_process_wrapper(OreProcessInput {
                    ore_exec: ore.to_str().unwrap_or_default().to_string(),
                    ore_threads: state.config.ore_threads.unwrap_or_default(),
                    ore_keypair: state.config.ore_keypair.clone().unwrap_or_default(),
                    ore_priority_fee: state.config.ore_priority_fee.unwrap_or_default(),
                    ore_rpc: state.config.ore_rpc.clone().unwrap_or_default(),
                    ore_tx: state.ore_tx.clone(),
                })
                .await
                {
                    Ok(result) => {
                        state.config.ore_status = Option::from(TaskStatus::Running);
                        state.ore_pid = Some(result.pid);
                        let _ = TOKIO_JOINER_TX.get().unwrap().send(result.handle);
                    }
                    Err(e) => {
                        tracing::error!("Error launching ore: {:?}", e);
                    }
                }
            }
        }
    }
    Ok(())
}
