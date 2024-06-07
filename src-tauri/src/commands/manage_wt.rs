use std::{
    fs, io,
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::ZipArchive;

#[tauri::command]
pub fn auto_detected_wt_install_path() -> Result<String, String> {
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
        paths_to_scan = vec![
            PathBuf::from("/home/user/.local/share/Steam/steamapps/common/War Thunder"),
            PathBuf::from("/usr/local/share/Steam/steamapps/common/War Thunder"),
        ];
    } else if cfg!(target_os = "macos") {
        paths_to_scan = vec![PathBuf::from(
            "/Users/user/Library/Application Support/Steam/steamapps/common/War Thunder",
        )];
    } else {
        paths_to_scan = vec![];
    }

    for path in paths_to_scan {
        if check_is_folder_contains_wt_launcher(&path) {
            return Ok(path.to_str().unwrap().to_string());
        }
    }
    return Err("War Thunder install path not found".to_string());
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

#[tauri::command]
pub fn install_user_skin(skin_path: String, wt_install_path: String) -> Result<(), String> {
    let skin_pb = PathBuf::from(&skin_path);
    if !skin_pb.exists() {
        return Err(format!("Path does not exist: {}", skin_pb.display()));
    }
    // 如果涂装文件夹不存在，则创建
    let skin_base_path = Path::new(&wt_install_path).join("UserSkins");
    if !skin_base_path.exists() {
        fs::create_dir_all(skin_base_path).unwrap();
    }
    // 首先检查path是路径还是文件夹
    // 如果是文件夹，则判断文件夹下是否有.blk文件，如果有，则复制这个文件夹到 wt_install_path 下
    if skin_pb.is_dir() {
        if !check_is_folder_contains_blk_file(&skin_pb) {
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
            if !check_is_folder_contains_blk_file(&extracted_folder) {
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

#[tauri::command]
pub fn check_is_valid_wt_install_path(path: String) -> bool {
    let path = Path::new(&path);
    check_is_folder_contains_wt_launcher(path)
}

// Check if the folder contains the War Thunder launcher
// TODO: Maybe not working on Linux and MacOS
fn check_is_folder_contains_wt_launcher(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    if !path.is_dir() {
        return false;
    }
    let launcher_names = vec!["launcher.exe"];

    for name in launcher_names {
        let launcher_path = path.join(name);
        if launcher_path.exists() {
            return true;
        }
    }
    return false;
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

fn check_is_folder_contains_blk_file(path: &PathBuf) -> bool {
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

#[derive(Debug, serde::Serialize)]
pub struct UserSkinInfo {
    vehicle_id: String,
    skin_name: String,
    full_path: String,
    resources: Vec<String>,
    folder_size: u64,
}

fn calculate_directory_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            total_size += entry.metadata()?.len();
        }
    }
    Ok(total_size)
}

#[tauri::command]
pub fn get_user_skins(wt_install_path: String) -> Result<Vec<UserSkinInfo>, String> {
    let skin_base_path = Path::new(&wt_install_path).join("UserSkins");

    if !skin_base_path.exists() {
        return Err("UserSkins folder not found".to_string());
    }
    let mut infos = Vec::new();
    for entry in WalkDir::new(&skin_base_path).min_depth(1).max_depth(10) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let mut blk_files = Vec::new();
            let mut resources = Vec::new();
            for inner_entry in WalkDir::new(entry.path()).min_depth(1).max_depth(1) {
                let inner_entry = inner_entry.unwrap();
                if inner_entry.file_type().is_file() {
                    if let Some(ext) = inner_entry.path().extension() {
                        if ext == "blk" {
                            blk_files.push(inner_entry);
                        } else {
                            resources.push(inner_entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }

            if blk_files.len() > 0 {
                let skin_base_path_str = skin_base_path.to_str().unwrap().to_string();
                let vehicle_id = blk_files[0]
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let skin_name = entry
                    .path()
                    .strip_prefix(&skin_base_path_str)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let full_path = entry.path().to_string_lossy().to_string();
                let folder_size = calculate_directory_size(entry.path()).unwrap();
                infos.push(UserSkinInfo {
                    vehicle_id,
                    skin_name,
                    full_path,
                    resources,
                    folder_size,
                });
            }
        }
    }
    Ok(infos)
}
