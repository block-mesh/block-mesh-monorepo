use crate::system_tray::set_dock_visible;
use tauri::{AppHandle, RunEvent};

pub fn on_run_events(_app_handle: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::Updater(updater_event) => {
            match updater_event {
                tauri::UpdaterEvent::UpdateAvailable {
                    body,
                    date,
                    version,
                } => {
                    tracing::info!("update available {} {:?} {}", body, date, version);
                }
                // Emitted when the download is about to be started.
                tauri::UpdaterEvent::Pending => {
                    tracing::info!("update is pending!");
                }
                tauri::UpdaterEvent::DownloadProgress {
                    chunk_length: _,
                    content_length: _,
                } => {
                    // println!("downloaded {} of {:?}", chunk_length, content_length);
                }
                // Emitted when the download has finished and the update is about to be installed.
                tauri::UpdaterEvent::Downloaded => {
                    tracing::info!("update has been downloaded!");
                }
                // Emitted when the update was installed. You can then ask to restart the app.
                tauri::UpdaterEvent::Updated => {
                    tracing::info!("app has been updated");
                }
                // Emitted when the app already has the latest version installed and an update is not needed.
                tauri::UpdaterEvent::AlreadyUpToDate => {
                    tracing::info!("app is already up to date");
                }
                // Emitted when there is an error with the updater. We suggest to listen to this event even if the default dialog is enabled.
                tauri::UpdaterEvent::Error(error) => {
                    tracing::error!("failed to update: {}", error);
                }
            }
        }
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
