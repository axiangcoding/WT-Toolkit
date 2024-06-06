use std::fs;
use std::path::Path;
use std::process::Command;

#[tauri::command]
pub fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args([
                // "/select,",
                &path,
            ]) // The comma after select is not a typo
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

#[tauri::command]
pub fn delete_folder(path: String) -> Result<(), String> {
    let path = Path::new(&path);

    // 检查路径是否存在
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    // 检查是否是目录
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }

    // 尝试删除目录及其内容
    match fs::remove_dir_all(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete folder: {}", err)),
    }
}

#[tauri::command]
pub fn create_folder(path: String)  {
    let path = Path::new(&path);

    // 检查路径是否存在
    if !path.exists() {
        // 尝试创建目录
        fs::create_dir_all(path).unwrap();
    }
}


