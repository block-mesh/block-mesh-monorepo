#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::let_underscore_future)]

use blockmesh_tauri_lib::run;
use std::process::ExitCode;
#[tokio::main]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
async fn main() -> anyhow::Result<ExitCode> {
    run()
}

#[tokio::main]
#[cfg(any(target_os = "android", target_os = "ios"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() -> anyhow::Result<ExitCode> {
    run()
}
