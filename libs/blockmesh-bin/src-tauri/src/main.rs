// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::let_underscore_future)]

use std::process::ExitCode;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, OnceLock};

use clap::Parser;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::cli::CliArgs;
use block_mesh_common::constants::DeviceType;
use logger_general::tracing::setup_tracing;

use crate::commands::{
    get_app_config, get_ore_status, get_task_status, open_main_window, set_app_config, toggle_miner,
};
use crate::handle_collectors::tokio_joiner_loop;
use crate::ore::ore_process_monitor::ore_process_monitor;
use crate::run_events::on_run_events;
use crate::system_tray::{on_system_tray_event, set_dock_visible, setup_tray};
use crate::tauri_state::{AppState, ChannelMessage};
use crate::tauri_storage::setup_storage;
use crate::windows_events::on_window_event;

mod background;
mod commands;
mod handle_collectors;
mod ore;
mod run_events;
mod system_tray;
mod tauri_state;
mod tauri_storage;
mod windows_events;

pub static TOKIO_JOINER_TX: OnceLock<Sender<tokio::task::JoinHandle<()>>> = OnceLock::new();
pub static SYSTEM: OnceLock<Mutex<sysinfo::System>> = OnceLock::new();

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let (incoming_tx, incoming_rx) = broadcast::channel::<ChannelMessage>(2);
    let args = CliArgs::parse();
    let mut config = if let Some(command) = args.command {
        AppConfig::from(command)
    } else {
        AppConfig::default()
    };
    config.device_id = config.device_id.or(Some(Uuid::new_v4()));
    setup_tracing(config.device_id.unwrap(), DeviceType::Desktop);
    let (ore_tx, ore_rx) = tokio::sync::mpsc::channel::<TaskStatus>(100);

    let (tokio_joiner_tx, tokio_joiner_rx) = mpsc::channel::<tokio::task::JoinHandle<()>>();
    let _ = TOKIO_JOINER_TX.set(tokio_joiner_tx);

    let app_state = Arc::new(Mutex::new(AppState {
        config,
        tx: incoming_tx,
        rx: incoming_rx,
        ore_pid: None,
        ore_tx,
    }));

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tokio::spawn(tokio_joiner_loop(tokio_joiner_rx));
    tokio::spawn(ore_process_monitor(ore_rx, app_state.clone()));
    let app_state = app_state.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(
            move |app, _argv, _cwd| {
                open_main_window(app).unwrap();
            },
        ))
        .system_tray(setup_tray())
        .manage(app_state.clone())
        .setup(move |app| {
            let app_handle = app.app_handle();
            let _: tauri::async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
                let _ = setup_storage(app_handle).await;
            });
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Accessory);
            }
            let app_handle = app.app_handle();
            if args.minimized {
                let window = app_handle.get_window("main").unwrap();
                window.hide().unwrap();
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
            get_task_status,
            toggle_miner,
            get_ore_status
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(on_run_events);
    Ok(ExitCode::SUCCESS)
}
