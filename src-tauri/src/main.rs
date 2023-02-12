use tauri::{
    Manager,
    CustomMenuItem,
    SystemTray,
    SystemTrayMenu,
    SystemTrayMenuItem,
    SystemTrayEvent,
    PhysicalPosition
};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position,
                size,
                ..
            } => {
                let w = app.get_window("main").unwrap();
                let visible = w.is_visible().unwrap();
                if visible {
                    w.hide().unwrap();
                } else {
                    let window_size  = w.outer_size().unwrap();
                    let physical_pos = PhysicalPosition {
                        x: position.x as i32 + (size.width as i32 / 2) - (window_size.width as i32 / 2),
                        y: position.y as i32 - window_size.height as i32
                    };
                    let _ = w.set_position(tauri::Position::Physical(physical_pos));
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
            },
            SystemTrayEvent::MenuItemClick {id, ..} => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    },
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        let _ = window.hide();
                    },
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}