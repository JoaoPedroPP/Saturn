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
    PhysicalPosition,
    Runtime,
    Window,
};
use auto_launch::{AutoLaunch, AutoLaunchBuilder};

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton, NSWindowStyleMask, NSWindowTitleVisibility};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
use objc::runtime::YES;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                title_transparent,
            );

            id.setStyleMask_(style_mask);

            if remove_tool_bar {
                let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
                let _: () = msg_send![close_button, setHidden: YES];
                let min_button = id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
                let _: () = msg_send![min_button, setHidden: YES];
                let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
                let _: () = msg_send![zoom_button, setHidden: YES];
            }

            id.setTitleVisibility_(if title_transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });

            id.setTitlebarAppearsTransparent_(if title_transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

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

#[tauri::command]
fn open_file_path(path: &str) -> bool {
    let curr_path = std::path::Path::new(path);
    let arg = curr_path;
  
    if cfg!(target_os = "macos") {
        std::process::Command::new("open")
        .args([arg])
        .output()
        .expect("failed to execute process");
        true
    } else {
        false
    }
  }

fn main() {
    let home: String = home_dir().unwrap().to_str().unwrap().to_string();
    let path = format!("{}/.ponto", home);
    let folder = Path::new(&path);
    match read_dir(folder) {
        Ok(mut x) => {
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
        .invoke_handler(tauri::generate_handler![mark, open_file_path])
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
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            window.set_transparent_titlebar(true, true);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
