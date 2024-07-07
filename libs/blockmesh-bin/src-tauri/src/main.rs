#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::let_underscore_future)]

use blockmesh_tauri_lib::run;
use std::process::ExitCode;
#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    run()
}
