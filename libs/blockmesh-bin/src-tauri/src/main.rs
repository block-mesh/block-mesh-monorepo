// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use auto_launch::AutoLaunchBuilder;
use block_mesh_common::cli::{CliArgs, Commands};
use block_mesh_common::constants::{BLOCKMESH_DISABLE_GUI_ENVAR, BLOCKMESH_HOME_DIR_ENVAR};
use clap::Parser;
use client_node::client_node_main;
use proxy_endpoint::proxy_endpoint_main;
use proxy_master::proxy_master_main;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use tauri::api::path::home_dir;
use tauri::utils::platform::current_exe;
use tauri::State;
use tokio::sync::Mutex;

type AppState = Arc<Mutex<CliArgs>>;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str, _state: State<'_, AppState>) -> Result<String, ()> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

pub fn setup_dir(exec: PathBuf, override_home_dir: Option<String>) -> String {
    let home = home_dir()
        .unwrap_or_else(|| {
            let root: PathBuf = if cfg!(target_os = "windows") {
                "C:".parse().unwrap()
            } else {
                "/".parse().unwrap()
            };
            let username = whoami::username();
            root.join(username)
        })
        .join("blockmesh");
    let home = if let Some(override_home_dir) = override_home_dir {
        Path::new(&override_home_dir).to_path_buf()
    } else {
        home
    };
    if !Path::new(&home).exists() {
        std::fs::create_dir_all(&home).unwrap();
    }
    let new_exec_path = home.join(exec.file_name().unwrap().to_str().unwrap());
    if !Path::new(&new_exec_path).exists() {
        std::fs::copy(&exec, &new_exec_path).unwrap();
    }
    new_exec_path.to_str().unwrap().to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let override_home_dir = std::env::var(BLOCKMESH_HOME_DIR_ENVAR).ok();
    let disable_gui = std::env::var(BLOCKMESH_DISABLE_GUI_ENVAR).ok();

    let current_exe_path = current_exe().unwrap();
    let exe = setup_dir(current_exe_path, override_home_dir);
    let auto = AutoLaunchBuilder::new()
        .set_app_name("BlockMesh Network")
        .set_app_path(&exe)
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    auto.enable().unwrap();
    auto.is_enabled().unwrap();

    let args = CliArgs::parse();
    let app_state: AppState = Arc::new(Mutex::new(args.clone()));
    if disable_gui.is_some() {
        tauri::Builder::default()
            .manage(app_state)
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    match &args.command {
        Some(commands) => match &commands {
            Commands::ClientNode(client_node_options) => {
                client_node_main(client_node_options).await
            }
            Commands::ProxyMaster(proxy_master_node_options) => {
                proxy_master_main(proxy_master_node_options).await
            }
            Commands::ProxyEndpoint(proxy_endpoint_node_options) => {
                proxy_endpoint_main(proxy_endpoint_node_options).await
            }
        },
        None => {
            println!("No command provided");
            Ok(ExitCode::SUCCESS)
        }
    }
}
