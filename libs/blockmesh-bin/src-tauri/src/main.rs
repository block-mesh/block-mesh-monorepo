// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::let_underscore_future)]
use crate::background::start_task;
use crate::commands::{get_app_config, get_task_status, open_main_window, set_app_config};
use crate::run_events::on_run_events;
use crate::system_tray::{on_system_tray_event, set_dock_visible, setup_tray};
use crate::tauri_state::{AppState, ChannelMessage};
use crate::tauri_storage::setup_storage;
use crate::windows_events::on_window_event;
use block_mesh_common::app_config::AppConfig;
use block_mesh_common::cli::CliArgs;
use block_mesh_common::constants::{BLOCKMESH_DISABLE_GUI_ENVAR, BLOCKMESH_HOME_DIR_ENVAR};
use clap::Parser;
use std::process::ExitCode;
use std::sync::Arc;
use tauri::utils::platform::current_exe;
use tauri::{ActivationPolicy, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, Mutex};

mod background;
mod commands;
mod run_events;
mod system_tray;
mod tauri_state;
mod tauri_storage;
mod windows_events;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let _override_home_dir = std::env::var(BLOCKMESH_HOME_DIR_ENVAR).ok();
    let _disable_gui = std::env::var(BLOCKMESH_DISABLE_GUI_ENVAR).ok();
    let (incoming_tx, incoming_rx) = broadcast::channel::<ChannelMessage>(2);
    let _current_exe_path = current_exe().unwrap();
    let rx = incoming_tx.subscribe();
    let args = CliArgs::parse();
    let config = if let Some(command) = args.command {
        AppConfig::from(command)
    } else {
        AppConfig::default()
    };

    let app_state = Arc::new(Mutex::new(AppState {
        config,
        tx: incoming_tx,
        rx: incoming_rx,
    }));
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let state = app_state.clone();
    tauri::async_runtime::spawn(async move { start_task(state.clone(), rx) });

    let app_state = app_state.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            open_main_window(app).unwrap();
        }))
        .system_tray(setup_tray())
        .manage(app_state.clone())
        .setup(move |app| {
            let app_handle = app.app_handle();
            let _: tauri::async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
                let _ = setup_storage(app_handle).await;
            });
            app.set_activation_policy(ActivationPolicy::Accessory);
            if args.minimized {
                set_dock_visible(false);
            } else {
                open_main_window(&app.app_handle()).unwrap();
            }
            Ok(())
        })
        .on_system_tray_event(on_system_tray_event)
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            get_app_config,
            set_app_config,
            get_task_status
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(on_run_events);
    Ok(ExitCode::SUCCESS)
}
