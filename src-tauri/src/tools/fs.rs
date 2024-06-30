use std::{fs, path::Path};

use fs_extra::dir::{copy, CopyOptions};
use zip::ZipArchive;

/// Copy a folder from the source path to the target path.
///
/// 将源路径的文件夹复制到目标路径。
pub fn copy_folder(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let options = CopyOptions::new().overwrite(true).copy_inside(true); //Initialize default values for CopyOptions

    // copy source/dir1 to target/dir1
    copy(from, to, &options)?;
    Ok(())
}

/// Decompress a file with extension zip, rar or 7z to the target path.
///
/// 解压缩扩展名为 zip、rar 或 7z 的文件到目标路径。
pub fn decompress_file(file_path: &Path, target_path: &Path) -> Result<(), String> {
    let extension: &str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match extension.to_lowercase().as_str() {
        "zip" => {
            let file = fs::File::open(&file_path)
                .map_err(|e| format!("Failed to open zip file: {}", e))?;
            let mut archive =
                ZipArchive::new(file).map_err(|e| format!("Failed to read zip file: {}", e))?;
            archive
                .extract(&target_path)
                .map_err(|e| format!("Failed to extract zip file: {}", e))?;
        }
        "rar" => return Err(format!("not support rar file now")),
        "7z" => {
            sevenz_rust::decompress_file(&file_path, &target_path)
                .map_err(|e| format!("Failed to extract 7z file: {}", e))?;
        }
        _ => return Err(format!("Unsupported file extension: {}", extension)),
    }
    Ok(())
}
