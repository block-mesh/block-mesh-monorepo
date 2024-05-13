use crate::state::{AppState, ChannelMessage};
use crate::system_tray::set_dock_visible;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn greet(name: &str, state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, ()> {
    println!("Greeting: {}", name);
    let state = state.lock().await;
    state.tx.send(ChannelMessage::RestartTask).unwrap();
    println!("Sent message");
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
pub fn open_main_window(app_handle: &AppHandle) -> anyhow::Result<()> {
    set_dock_visible(true);
    if let Some(window) = app_handle.get_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let _window = tauri::WindowBuilder::new(
            app_handle,
            "main",
            tauri::WindowUrl::App("index.html".into()),
        )
        .visible(false)
        .build()?;
    }
    Ok(())
}
