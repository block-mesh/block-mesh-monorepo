use tauri::GlobalWindowEvent;

pub fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            // event.window().minimize().unwrap();
            api.prevent_close();
        }
        _ => {
            // println!("window event: {:?}", event.event());
        }
    }
}
