use std::fmt::{Display, Formatter};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::menu::{MenuBuilder, MenuItemBuilder};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::{App, Manager};

pub enum MenuItems {
    Quit,
    Hide,
    Show,
    Invalid,
}

impl Display for MenuItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuItems::Hide => f.write_str("Hide"),
            MenuItems::Quit => f.write_str("Quit"),
            MenuItems::Show => f.write_str("Show"),
            MenuItems::Invalid => f.write_str("Invalid"),
        }
    }
}

impl From<String> for MenuItems {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "Hide" => MenuItems::Hide,
            "Quit" => MenuItems::Quit,
            "Show" => MenuItems::Show,
            _ => MenuItems::Invalid,
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn setup_tray(app: &App) {
    let quit = MenuItemBuilder::new(MenuItems::Quit.to_string())
        .id(MenuItems::Quit)
        .build(app)
        .unwrap();
    let hide = MenuItemBuilder::new(MenuItems::Hide.to_string())
        .id(MenuItems::Hide)
        .build(app)
        .unwrap();
    let show = MenuItemBuilder::new(MenuItems::Show.to_string())
        .id(MenuItems::Show)
        .build(app)
        .unwrap();

    let menu = MenuBuilder::new(app)
        .items(&[&quit, &hide, &show])
        .build()
        .unwrap();

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            let e = MenuItems::from(event.id().0.clone());
            match e {
                MenuItems::Quit => app.exit(0),
                MenuItems::Hide => {
                    let window = app.get_webview_window("main").unwrap();
                    window.hide().unwrap();
                }
                MenuItems::Show => {
                    let window = app.get_webview_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray_icon, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                if button == MouseButton::Left {
                    let window = tray_icon.app_handle().get_webview_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
        })
        .build(app)
        .unwrap();
}

pub fn set_dock_visible(_visible: bool) {
    // https://github.dev/thewh1teagle/RustDuck/tree/main/src-tauri/src
    // if cfg!(target_os = "macos") {
    //     let policy = if visible {
    //         NSApplicationActivationPolicyRegular
    //     } else {
    //         NSApplicationActivationPolicyAccessory
    //     };
    //     unsafe {
    //         let app = NSApp();
    //         app.setActivationPolicy_(policy);
    //     }
    // }
}
