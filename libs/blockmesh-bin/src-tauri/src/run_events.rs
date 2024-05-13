use crate::system_tray::set_dock_visible;
use tauri::{AppHandle, RunEvent};

pub fn on_run_events(_app_handle: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
            set_dock_visible(false);
            // for (_label, window) in app_handle.webview_windows() {
            //     window.close().unwrap();
            // }
        }
        RunEvent::MainEventsCleared => {}
        _ => {
            // println!("run event: {:?}", event);
        }
    }
}
