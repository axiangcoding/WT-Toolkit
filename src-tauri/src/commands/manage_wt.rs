use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use zip::ZipArchive;

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
                if !check_is_valid_folder_with_blk(&path) {
                    continue;
                }
                let vehicle_id = get_blk_name_in_folder(&path);
                let folder_name = path.file_name().unwrap().to_string_lossy().to_string();
                let size_kb = get_folder_size(&path);
                let mut folder_map = HashMap::new();
                folder_map.insert("name".to_string(), folder_name);
                folder_map.insert("size_bytes".to_string(), size_kb.to_string());
                folder_map.insert("path".to_string(), path.to_str().unwrap().to_string());
                folder_map.insert("vehicle_id".to_string(), vehicle_id);
                folder_info_vec.push(folder_map);
            }
        }
    }

    return folder_info_vec;
}

#[tauri::command]
pub fn install_user_skin(skin_path: String, wt_install_path: String) -> Result<(), String> {
    // 首先检查path是路径还是文件夹
    // 如果是文件夹，则判断文件夹下是否有.blk文件，如果有，则复制这个文件夹到 wt_install_path 下

    let skin_pb = PathBuf::from(&skin_path);
    if !skin_pb.exists() {
        return Err(format!("Path does not exist: {}", skin_pb.display()));
    }
    if skin_pb.is_dir() {
        if !check_is_valid_folder_with_blk(&skin_pb) {
            return Err(format!("Invalid skin folder: {}", skin_pb.display()));
        }
        let new_path = Path::new(&wt_install_path)
            .join("UserSkins")
            .join(skin_pb.file_name().unwrap());
        if new_path.exists() {
            return Err(format!(
                "Skin folder already exists: {}",
                new_path.display()
            ));
        }
        // 文件夹及文件夹下的内容均复制过去
        match copy_everything_to(&skin_pb, &new_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to copy skin folder: {}", err)),
        }
    } else {
        // 处理压缩文件
        let extension: &str = skin_pb.extension().and_then(|e| e.to_str()).unwrap_or("");
        let temp_dir =
            tempfile::tempdir().map_err(|e| format!("Failed to create temp dir: {}", e))?;
        let temp_path = temp_dir.path();

        match extension.to_lowercase().as_str() {
            "zip" => {
                let file = fs::File::open(&skin_pb)
                    .map_err(|e| format!("Failed to open zip file: {}", e))?;
                let mut archive =
                    ZipArchive::new(file).map_err(|e| format!("Failed to read zip file: {}", e))?;
                archive
                    .extract(&temp_path)
                    .map_err(|e| format!("Failed to extract zip file: {}", e))?;
            }
            "rar" => return Err(format!("not support rar file now")),
            "7z" => {
                sevenz_rust::decompress_file(&skin_pb, &temp_path)
                    .map_err(|e| format!("Failed to extract 7z file: {}", e))?;
            }
            _ => return Err(format!("Unsupported file extension: {}", extension)),
        }

        // 查找解压后的文件夹
        let mut extracted_folder = None;
        for entry in
            fs::read_dir(&temp_path).map_err(|e| format!("Failed to read temp dir: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read temp dir entry: {}", e))?;
            if entry.path().is_dir() {
                extracted_folder = Some(entry.path());
                break;
            }
        }

        if let Some(extracted_folder) = extracted_folder {
            if !check_is_valid_folder_with_blk(&extracted_folder) {
                return Err(format!(
                    "Invalid skin folder in archive: {}",
                    extracted_folder.display()
                ));
            }
            let new_path = Path::new(&wt_install_path)
                .join("UserSkins")
                .join(extracted_folder.file_name().unwrap());
            if new_path.exists() {
                return Err(format!(
                    "Skin folder already exists: {}",
                    new_path.display()
                ));
            }
            match copy_everything_to(&extracted_folder, &new_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Failed to copy extracted skin folder: {}", err)),
            }
        } else {
            Err("No folder found in extracted archive".to_string())
        }
    }
}

fn copy_everything_to(old_path: &Path, new_path: &Path) -> io::Result<()> {
    if old_path.is_dir() {
        fs::create_dir(new_path)?;
        for entry in fs::read_dir(old_path)? {
            let entry = entry?;
            let path = entry.path();
            let new_path = new_path.join(path.file_name().unwrap());
            copy_everything_to(&path, &new_path)?;
        }
    } else {
        fs::copy(old_path, new_path)?;
    }
    Ok(())
}

fn check_is_valid_folder_with_blk(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }
    if !path.is_dir() {
        return false;
    }
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "blk" {
                    return true;
                }
            }
        }
    }
    return false;
}

fn get_blk_name_in_folder(path: &PathBuf) -> String {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "blk" {
                    return path.file_stem().unwrap().to_string_lossy().to_string();
                }
            }
        }
    }
    return "".to_string();
}
