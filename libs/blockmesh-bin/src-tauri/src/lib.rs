use std::env;
use std::process::ExitCode;
use std::sync::{Arc, OnceLock};

use clap::Parser;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

use block_mesh_common::app_config::AppConfig;
use block_mesh_common::cli::CliArgs;
use block_mesh_common::constants::DeviceType;
use logger_general::tracing::setup_tracing;

use crate::background::channel_receiver;
use crate::commands::{
    check_token, get_app_config, get_home_url, get_ore_status, get_task_status, login, logout,
    open_main_window, register, set_app_config, toggle_miner,
};
use crate::run_events::on_run_events;
use crate::system_tray::{set_dock_visible, setup_tray};
use crate::tauri_state::{AppState, ChannelMessage};
use crate::tauri_storage::setup_storage;
use crate::windows_events::on_window_event;

mod background;
mod blockmesh;
mod commands;
mod ore;
mod run_events;
mod system_tray;
mod tauri_state;
mod tauri_storage;
mod windows_events;
pub static SYSTEM: OnceLock<Mutex<sysinfo::System>> = OnceLock::new();

pub static CHANNEL_MSG_TX: OnceLock<broadcast::Sender<ChannelMessage>> = OnceLock::new();
pub static APP_ENVIRONMENT: OnceLock<String> = OnceLock::new();

const DEV_ENV: [&str; 3] = ["dev", "development", "local"];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<ExitCode> {
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_default();
    APP_ENVIRONMENT.set(app_environment).unwrap();
    let (incoming_tx, incoming_rx) = broadcast::channel::<ChannelMessage>(2);
    let args = CliArgs::parse();
    let mut config = if let Some(command) = args.command {
        AppConfig::from(command)
    } else {
        AppConfig::default()
    };
    config.device_id = config.device_id.or(Some(Uuid::new_v4()));
    setup_tracing(config.device_id.unwrap(), DeviceType::Desktop);

    let _ = CHANNEL_MSG_TX.set(incoming_tx.clone());

    let app_state = Arc::new(Mutex::new(AppState {
        config,
        tx: incoming_tx,
        rx: incoming_rx.resubscribe(),
        ore_pid: None,
        node_handle: None,
        uptime_handle: None,
        task_puller: None,
    }));

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tokio::spawn(channel_receiver(incoming_rx, app_state.clone()));
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
        .manage(app_state.clone())
        .setup(move |app| {
            setup_tray(app);
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
            }
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Accessory);
            }
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(setup_storage(app_handle.clone()));
            let app_handle = app.app_handle();
            if args.minimized {
                let window = app_handle.get_webview_window("main").unwrap();
                window.hide().unwrap();
                set_dock_visible(false);
            } else {
                open_main_window(app.app_handle()).unwrap();
            }
            Ok(())
        })
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            get_app_config,
            set_app_config,
            get_task_status,
            toggle_miner,
            get_ore_status,
            login,
            register,
            check_token,
            logout,
            get_home_url
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(on_run_events);
    Ok(ExitCode::SUCCESS)
}
