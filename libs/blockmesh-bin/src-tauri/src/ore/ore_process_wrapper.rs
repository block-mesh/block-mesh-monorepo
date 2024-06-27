use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use sysinfo::{Pid, System};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use block_mesh_common::app_config::TaskStatus;

use crate::SYSTEM;

#[derive(Debug)]
pub struct OreProcessInput {
    pub ore_exec: String,
    pub ore_threads: u16,
    pub ore_keypair: String,
    pub ore_priority_fee: u64,
    pub ore_rpc: String,
    pub ore_tx: tokio::sync::mpsc::Sender<TaskStatus>,
}

#[derive(Debug)]
pub struct OreProcessOutput {
    pub pid: u32,
    pub handle: JoinHandle<()>,
    pub stdout_log: String,
    pub stderr_log: String,
}

#[tracing::instrument(name = "ore_process_wrapper", skip(params), ret, err)]
pub async fn ore_process_wrapper(params: OreProcessInput) -> anyhow::Result<OreProcessOutput> {
    let temp = env::temp_dir();
    let stdout_log_file = PathBuf::new().join(temp.clone()).join("ore_stdout.log");
    if tokio::fs::metadata(&stdout_log_file).await.is_err() {
        tokio::fs::write(&stdout_log_file, "").await?
    }
    let stderr_log_file = PathBuf::new().join(temp).join("ore_stderr.log");
    if tokio::fs::metadata(&stderr_log_file).await.is_err() {
        tokio::fs::write(&stderr_log_file, "").await?;
    }
    let stdout_log = stdout_log_file.to_str().unwrap().to_string();
    let stderr_log = stderr_log_file.to_str().unwrap().to_string();
    let stdout_log_file = File::open(&stdout_log_file)?;
    let stderr_log_file = File::open(&stderr_log_file)?;
    let mut child = Command::new(&params.ore_exec)
        .arg("mine")
        .arg("--rpc")
        .arg(params.ore_rpc)
        .arg("--threads")
        .arg(params.ore_threads.to_string())
        .arg("--keypair")
        .arg(params.ore_keypair)
        .arg("--priority-fee")
        .arg(params.ore_priority_fee.to_string())
        .stdout(Stdio::from(stdout_log_file))
        .stderr(Stdio::from(stderr_log_file))
        .spawn()?;

    let pid = child.id().ok_or(anyhow::anyhow!("Failed to get PID"))?;
    let handle: JoinHandle<()> = tokio::spawn(async move {
        let _ = child.wait().await;
        params.ore_tx.send(TaskStatus::Off).await.unwrap();
    });

    Ok(OreProcessOutput {
        pid,
        handle,
        stdout_log,
        stderr_log,
    })
}

#[tracing::instrument(name = "kill_ore_process", ret, err)]
pub async fn kill_ore_process(pid: u32) -> anyhow::Result<()> {
    let system = SYSTEM.get_or_init(|| Mutex::new(System::new_all()));
    let mut system = system.lock().await;
    system.refresh_process(Pid::from_u32(pid));
    system
        .processes_by_name("ore")
        .filter(|process| process.pid().as_u32() == pid)
        .for_each(|process| {
            process.kill();
        });
    Ok(())
}
