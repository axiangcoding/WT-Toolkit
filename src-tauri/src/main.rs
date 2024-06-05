// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::vec;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn auto_detected_wt_install_path() -> String {
    // 定义一个列表
    let paths_to_scan = vec![
        "C://Program Files/Steam/steamapps/common/War Thunder",
        "D://Program Files/Steam/steamapps/common/War Thunder",
        "E://Program Files/Steam/steamapps/common/War Thunder",
        "F://Program Files/Steam/steamapps/common/War Thunder",
        "G://Program Files/Steam/steamapps/common/War Thunder",
        "H://Program Files/Steam/steamapps/common/War Thunder",
    ];

    // 遍历列表
    for path_str in paths_to_scan {
        let path = std::path::Path::new(path_str);
        if path.exists() {
            // 检查目录下的launcher.exe是否存在
            let launcher_path = format!("{}/launcher.exe", path_str);
            if std::path::Path::new(&launcher_path).exists() {
                return path.to_str().unwrap().to_string();
            }
        }
    }
    return "".to_string();
}

#[tauri::command]
fn auto_detected_wt_setting_path() -> String {
    // 写一个rust代码，判断 %USERPROFILE%\Documents\My Games\WarThunder\Saves存不存在
    let user_profile = std::env::var("USERPROFILE").unwrap();
    let setting_path = format!("{}/Documents/My Games/WarThunder/Saves", user_profile);
    let path: &std::path::Path = std::path::Path::new(&setting_path);
    if path.exists() {
        return path.to_str().unwrap().to_string();
    }
    return "".to_string();
}

#[tauri::command]
fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            if let Ok(Fork::Child) = daemon(false, false) {
                Command::new("dbus-send")
                    .args([
                        "--session",
                        "--dest=org.freedesktop.FileManager1",
                        "--type=method_call",
                        "/org/freedesktop/FileManager1",
                        "org.freedesktop.FileManager1.ShowItems",
                        format!("array:string:\"file://{path}\"").as_str(),
                        "string:\"\"",
                    ])
                    .spawn()
                    .unwrap();
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            show_in_folder,
            auto_detected_wt_install_path,
            auto_detected_wt_setting_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
