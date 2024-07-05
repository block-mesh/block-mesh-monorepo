use tauri::{Window, WindowEvent};

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
