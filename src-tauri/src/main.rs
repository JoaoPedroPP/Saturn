use std::path::Path;
use dirs::home_dir;
use std::io::Write;
use std::fs::{
    read_dir,
    OpenOptions,
    create_dir_all
};
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

#[tauri::command]
fn mark(state: &str, time: &str) -> String {
    let home: String = home_dir().unwrap().to_str().unwrap().to_string();
    let path = format!("{}/.ponto", home);
    let file_path = format!("{}/.ponto/ponto.csv", home);
    let mark = OpenOptions::new().create(true).read(true).write(true).append(true).open(file_path);
    let record = format!("{:?},{:?}\n", time, state);
    let _ = mark.expect("Não criou/abriu o arquivo.").write(record.as_bytes());
    format!("Marcado!")
}

fn main() {
    let home: String = home_dir().unwrap().to_str().unwrap().to_string();
    let path = format!("{}/.ponto", home);
    let folder = Path::new(&path);
    match read_dir(folder) {
        Ok(mut x) => {
            println!("existe dir. Files: {:?}", x);
            match x.find(|file| file.as_ref().unwrap().file_name() == "ponto.csv") {
                Some(f) => {
                    OpenOptions::new().read(true).append(true).open(f.unwrap().path());
                },
                None => {
                    let file_path = format!("{}/.ponto/ponto.csv", home);
                    let mark = OpenOptions::new().create(true).read(true).write(true).append(true).open(file_path);
                    let _ = mark.as_ref().expect("Não criou/abriu o arquivo.").write(b"Time,State\n");
                }
            }
        },
        Err(_) => {
            let _createdir = create_dir_all(folder).unwrap();
            let file_path = format!("{}/.ponto/ponto.csv", home);
            let mark = OpenOptions::new().create(true).read(true).write(true).append(true).open(file_path);
            let _ = mark.as_ref().expect("Não criou/abriu o arquivo.").write(b"Time,State\n");
        }
    };
    
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, mark])
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
