use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use log::warn;

use crate::{ret_code::RetCode, tools, WrappedState};

#[tauri::command]
pub fn auto_detected_wt_root_path() -> Result<String, RetCode> {
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
    return Err(RetCode::AutoDetectedWtRootPathFailed);
}

#[tauri::command]
pub fn auto_detected_wt_setting_path() -> Result<String, RetCode> {
    let user_profile = match std::env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_) => return Err(RetCode::AutoDetectedWtSettingPathFailed),
    };
    let path = std::path::Path::new(&user_profile)
        .join("Documents")
        .join("My Games")
        .join("WarThunder")
        .join("Saves");
    if path.exists() {
        return Ok(path.to_str().unwrap().to_string());
    }
    return Err(RetCode::AutoDetectedWtSettingPathFailed);
}

#[tauri::command]
pub fn install_user_skin(
    skin_path: String,
    state: tauri::State<WrappedState>,
) -> Result<(), RetCode> {
    let wt_root_path = state
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .config
        .clone()
        .wt_root_path
        .unwrap();

    let skin_pb = PathBuf::from(&skin_path);
    if !skin_pb.exists() {
        warn!("Skin path not exists: {:?}", skin_pb);
        return Err(RetCode::InstallUserSkinFailed);
    }
    // 如果涂装文件夹不存在，则创建
    let skin_base_path = Path::new(&wt_root_path).join("UserSkins");
    if !skin_base_path.exists() {
        warn!("Skin folder not found: {:?}", skin_base_path);
        fs::create_dir_all(skin_base_path).unwrap();
    }
    // 首先检查path是路径还是文件夹
    // 如果是文件夹，则判断文件夹下是否有.blk文件，如果有，则复制这个文件夹到 wt_install_path 下
    if skin_pb.is_dir() {
        if !check_is_folder_contains_blk_file(&skin_pb) {
            warn!("Skin folder not contains blk file: {:?}", skin_pb);
            return Err(RetCode::InstallUserSkinFailed);
        }
        let new_path = Path::new(&wt_root_path)
            .join("UserSkins")
            .join(skin_pb.file_name().unwrap());
        if new_path.exists() {
            warn!("Skin folder already exists: {:?}", new_path);
            return Err(RetCode::InstallUserSkinFailed);
        }
        match tools::fs::copy_folder(&skin_pb, &new_path) {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("Copy skin folder failed: {:?}", err);
                Err(RetCode::InstallUserSkinFailed)
            }
        }
    } else {
        // 处理压缩文件
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();

        tools::fs::decompress_file(skin_pb.as_path(), temp_path).unwrap();

        // 查找解压后的文件夹
        let mut extracted_folder = None;
        for entry in fs::read_dir(&temp_path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                extracted_folder = Some(entry.path());
                break;
            }
        }

        if let Some(extracted_folder) = extracted_folder {
            if !check_is_folder_contains_blk_file(&extracted_folder) {
                warn!(
                    "Extracted folder not contains blk file: {:?}",
                    extracted_folder
                );
                return Err(RetCode::InstallUserSkinFailed);
            }
            let new_path = Path::new(&wt_root_path)
                .join("UserSkins")
                .join(extracted_folder.file_name().unwrap());
            if new_path.exists() {
                warn!("Skin folder already exists: {:?}", new_path);
                return Err(RetCode::InstallUserSkinFailed);
            }
            match tools::fs::copy_folder(&extracted_folder, &new_path) {
                Ok(_) => Ok(()),
                Err(err) => {
                    warn!("Copy skin folder failed: {:?}", err);
                    Err(RetCode::InstallUserSkinFailed)
                }
            }
        } else {
            warn!("Extracted folder not found");
            Err(RetCode::InstallUserSkinFailed)
        }
    }
}

#[tauri::command]
pub fn install_user_sight(
    sight_path: String,
    state: tauri::State<WrappedState>,
) -> Result<(), RetCode> {
    let wt_root_path = state
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .config
        .clone()
        .wt_root_path
        .unwrap();

    let sight_pb = PathBuf::from(&sight_path);
    if !sight_pb.exists() {
        warn!("Sight path not exists: {:?}", sight_pb);
        return Err(RetCode::InstallUserSightFailed);
    }

    let sight_base_path = Path::new(&wt_root_path).join("UserSights");
    if !sight_base_path.exists() {
        fs::create_dir_all(sight_base_path).unwrap();
    }

    if sight_pb.is_dir() {
        if !check_is_folder_contains_blk_file(&sight_pb) {
            warn!("Sight folder not contains blk file: {:?}", sight_pb);
            return Err(RetCode::InstallUserSightFailed);
        }
        let new_path = Path::new(&wt_root_path)
            .join("UserSights")
            .join(sight_pb.file_name().unwrap());
        if new_path.exists() {
            warn!("Sight folder already exists: {:?}", new_path);
            return Err(RetCode::InstallUserSightFailed);
        }
        match tools::fs::copy_folder(&sight_pb, &new_path) {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("Copy sight folder failed: {:?}", err);
                Err(RetCode::InstallUserSightFailed)
            }
        }
    } else {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();

        tools::fs::decompress_file(sight_pb.as_path(), temp_path).unwrap();

        // 查找解压后的文件夹
        let mut extracted_folder = None;
        for entry in fs::read_dir(&temp_path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                extracted_folder = Some(entry.path());
                break;
            }
        }

        if let Some(extracted_folder) = extracted_folder {
            if !check_is_folder_contains_blk_file(&extracted_folder) {
                warn!(
                    "Extracted folder not contains blk file: {:?}",
                    extracted_folder
                );
                return Err(RetCode::InstallUserSightFailed);
            }
            let new_path = Path::new(&wt_root_path)
                .join("UserSights")
                .join(extracted_folder.file_name().unwrap());
            if new_path.exists() {
                warn!("Sight folder already exists: {:?}", new_path);
                return Err(RetCode::InstallUserSightFailed);
            }
            match tools::fs::copy_folder(&extracted_folder, &new_path) {
                Ok(_) => Ok(()),
                Err(_err) => {
                    warn!("Copy sight folder failed: {:?}", _err);
                    Err(RetCode::InstallUserSightFailed)
                }
            }
        } else {
            warn!("Extracted folder not found");
            Err(RetCode::InstallUserSightFailed)
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UserSkinInfo {
    vehicle_id: String,
    skin_name: String,
    full_path: String,
    resources: Vec<String>,
    folder_size: u64,
}

#[tauri::command]
pub fn get_user_skins(state: tauri::State<WrappedState>) -> Result<Vec<UserSkinInfo>, RetCode> {
    let wt_root_path = state
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .config
        .clone()
        .wt_root_path
        .unwrap();

    let skin_base_path = Path::new(&wt_root_path).join("UserSkins");

    if !skin_base_path.exists() {
        warn!("UserSkins folder not found");
        return Err(RetCode::GetUserSkinsFailed);
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
                let folder_size = fs_extra::dir::get_size(entry.path()).unwrap();
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

#[derive(Debug, serde::Serialize)]
pub struct UserSightInfo {
    vehicle_id: String,
    folder_name: String,
    full_path: String,
    sight_names: Vec<String>,
    folder_size: u64,
}

#[tauri::command]
pub fn get_user_sights(state: tauri::State<WrappedState>) -> Result<Vec<UserSightInfo>, RetCode> {
    let wt_root_path = state
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .config
        .clone()
        .wt_root_path
        .unwrap();

    let skin_base_path = Path::new(&wt_root_path).join("UserSights");

    if !skin_base_path.exists() {
        warn!("UserSights folder not found");
        return Err(RetCode::GetUserSightsFailed);
    }
    let mut infos = Vec::new();
    for entry in WalkDir::new(&skin_base_path).min_depth(1).max_depth(10) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let mut sight_names = Vec::new();
            for inner_entry in WalkDir::new(entry.path()).min_depth(1).max_depth(1) {
                let inner_entry = inner_entry.unwrap();
                if inner_entry.file_type().is_file() {
                    if let Some(ext) = inner_entry.path().extension() {
                        if ext == "blk" {
                            sight_names.push(
                                inner_entry
                                    .path()
                                    .file_stem()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string(),
                            );
                        }
                    }
                }
            }

            if sight_names.len() > 0 {
                let skin_base_path_str = skin_base_path.to_str().unwrap().to_string();
                let vehicle_id = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let folder_name = entry
                    .path()
                    .strip_prefix(&skin_base_path_str)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let full_path = entry.path().to_string_lossy().to_string();
                let folder_size = fs_extra::dir::get_size(entry.path()).unwrap();
                infos.push(UserSightInfo {
                    vehicle_id,
                    folder_name,
                    full_path,
                    sight_names,
                    folder_size,
                });
            }
        }
    }
    Ok(infos)
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
