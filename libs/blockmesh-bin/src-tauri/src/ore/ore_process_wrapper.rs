use anyhow::anyhow;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use sysinfo::{Pid, System};
// use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use block_mesh_common::app_config::TaskStatus;

use crate::tauri_state::AppState;
use crate::SYSTEM;

#[derive(Debug)]
pub struct OreProcessInput {
    pub ore_exec: String,
    pub ore_threads: u16,
    pub ore_keypair: String,
    pub ore_priority_fee: u64,
    pub ore_rpc: String,
}

#[derive(Debug)]
pub struct OreProcessOutput {
    pub pid: u32,
    pub handle: JoinHandle<()>,
    pub stdout_log: String,
    pub stderr_log: String,
}

#[tracing::instrument(name = "ore_process_wrapper", skip(app_state), ret, err)]
pub async fn start_ore_process(app_state: Arc<Mutex<AppState>>) -> anyhow::Result<()> {
    let mut state = app_state.lock().await;

    let exec = env::current_exe().unwrap_or_default();
    let ore = exec.parent().unwrap().join("resources").join("ore");
    if tokio::fs::metadata(&ore).await.is_err() {
        log::error!("ORE binary not found at {:?}", ore);
        state.config.ore_status = Option::from(TaskStatus::Off);
        return Err(anyhow!("ORE binary not found"));
    }
    let ore_exec = ore.to_str().unwrap_or_default().to_string();
    let ore_threads = state.config.ore_threads.unwrap_or_default();
    let ore_keypair = state.config.ore_keypair.clone().unwrap_or_default();
    let ore_priority_fee = state.config.ore_priority_fee.unwrap_or_default();
    let ore_rpc = state.config.ore_rpc.clone().unwrap_or_default();

    let temp = env::temp_dir();
    let stdout_log_file = PathBuf::new().join(temp.clone()).join("ore_stdout.log");
    if tokio::fs::metadata(&stdout_log_file).await.is_err() {
        tokio::fs::write(&stdout_log_file, "").await?
    }
    let stderr_log_file = PathBuf::new().join(temp).join("ore_stderr.log");
    if tokio::fs::metadata(&stderr_log_file).await.is_err() {
        tokio::fs::write(&stderr_log_file, "").await?;
    }
    let _stdout_log = stdout_log_file.to_str().unwrap().to_string();
    let _stderr_log = stderr_log_file.to_str().unwrap().to_string();
    let stdout_log_file = File::open(&stdout_log_file)?;
    let stderr_log_file = File::open(&stderr_log_file)?;
    let mut child = Command::new(&ore_exec)
        .arg("mine")
        .arg("--rpc")
        .arg(ore_rpc)
        .arg("--threads")
        .arg(ore_threads.to_string())
        .arg("--keypair")
        .arg(ore_keypair)
        .arg("--priority-fee")
        .arg(ore_priority_fee.to_string())
        // .stdout(Stdio::piped())
        .stdout(Stdio::from(stdout_log_file))
        .stderr(Stdio::from(stderr_log_file))
        .spawn()?;

    // let stdout = child
    //     .stdout
    //     .take()
    //     .expect("child did not have a handle to stdout");
    // let mut reader = BufReader::new(stdout).lines();
    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    // tokio::spawn(async move {
    //     while let Some(line) = reader.next_line().await.unwrap_or_default() {
    //         tracing::info!("ORE Line: {}", line);
    //     }
    // });

    let pid = child.id().ok_or(anyhow::anyhow!("Failed to get PID"))?;
    tokio::spawn(async move {
        let _ = child.wait().await;
    });
    state.config.ore_status = Option::from(TaskStatus::Running);
    state.ore_pid = Some(pid);
    Ok(())
}

#[tracing::instrument(name = "kill_ore_process", skip(app_state), ret, err)]
pub async fn kill_ore_process(app_state: Arc<Mutex<AppState>>) -> anyhow::Result<()> {
    let mut state = app_state.lock().await;
    if let Some(pid) = state.ore_pid {
        let system = SYSTEM.get_or_init(|| Mutex::new(System::new_all()));
        let mut system = system.lock().await;
        system.refresh_process(Pid::from_u32(pid));
        system
            .processes_by_name("ore")
            .filter(|process| process.pid().as_u32() == pid)
            .for_each(|process| {
                process.kill();
            });
        state.config.ore_status = Option::from(TaskStatus::Off);
        state.ore_pid = None;
    }
    Ok(())
}
