#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::{Window, WindowEvent};

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            // event.window().minimize().unwrap();
            api.prevent_close();
        }
        _ => {
            // println!("window event: {:?}", event.event());
        }
    }
}
