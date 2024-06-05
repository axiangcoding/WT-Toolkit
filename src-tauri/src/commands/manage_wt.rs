use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[tauri::command]
pub fn auto_detected_wt_install_path() -> String {
    // 定义一个列表
    let paths_to_scan: Vec<PathBuf>;

    if cfg!(target_os = "windows") {
        paths_to_scan = vec![
            PathBuf::from("C:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
            PathBuf::from("D:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
            PathBuf::from("E:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
            PathBuf::from("F:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
            PathBuf::from("G:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
            PathBuf::from("H:\\Program Files\\Steam\\steamapps\\common\\War Thunder"),
        ];
    } else if cfg!(target_os = "linux") {
        // 在Linux下常见的Steam路径
        paths_to_scan = vec![
            PathBuf::from("/home/user/.local/share/Steam/steamapps/common/War Thunder"),
            PathBuf::from("/usr/local/share/Steam/steamapps/common/War Thunder"),
        ];
    } else if cfg!(target_os = "macos") {
        // 在MacOS下常见的Steam路径
        paths_to_scan = vec![PathBuf::from(
            "/Users/user/Library/Application Support/Steam/steamapps/common/War Thunder",
        )];
    } else {
        paths_to_scan = vec![];
    }

    // 遍历列表
    for path in paths_to_scan {
        if path.exists() {
            // 检查目录下的launcher.exe（在Linux上可能是launcher或其他可执行文件）是否存在
            let launcher_path = path.join("launcher.exe"); // 在Linux上这个可能是不同的
            if launcher_path.exists() {
                return path.to_str().unwrap().to_string();
            }
        }
    }
    "".to_string()
}

#[tauri::command]
pub fn auto_detected_wt_setting_path() -> String {
    // 写一个rust代码，判断 %USERPROFILE%\Documents\My Games\WarThunder\Saves存不存在
    let user_profile = std::env::var("USERPROFILE").unwrap();
    let path = std::path::Path::new(&user_profile)
        .join("Documents")
        .join("My Games")
        .join("WarThunder")
        .join("Saves");
    if path.exists() {
        return path.to_str().unwrap().to_string();
    }
    return "".to_string();
}

fn get_folder_size(path: &Path) -> u64 {
    let mut size = 0;

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            size += get_folder_size(&path);
        } else {
            size += entry.metadata().unwrap().len();
        }
    }

    return size;
}

#[tauri::command]
pub fn get_user_skins_info(path: String) -> Vec<HashMap<String, String>> {
    let mut folder_info_vec = Vec::new();
    let dir = Path::new(&path).join("UserSkins");
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            // 如果是文件夹，并且文件夹下有以.blk后缀的文件，就认为是皮肤文件夹

            if path.is_dir() {
                let mut has_blk_file = false;
                for entry in fs::read_dir(&path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "blk" {
                                has_blk_file = true;
                                break;
                            }
                        }
                    }
                }
                if !has_blk_file {
                    continue;
                }

                let folder_name = path.file_name().unwrap().to_string_lossy().to_string();
                let size_kb = get_folder_size(&path);
                let mut folder_map = HashMap::new();
                folder_map.insert("name".to_string(), folder_name);
                folder_map.insert("size_bytes".to_string(), size_kb.to_string());
                folder_map.insert("path".to_string(), path.to_str().unwrap().to_string());
                folder_info_vec.push(folder_map);
            }
        }
    }

    return folder_info_vec;
}
