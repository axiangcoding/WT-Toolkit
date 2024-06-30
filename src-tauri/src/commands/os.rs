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
        Command::new("xdg-open").args([path]).spawn().unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

#[tauri::command]
pub fn delete_folder(path: String) -> Result<(), String> {
    fs_extra::dir::remove(path).map_err(|e| e.to_string())
}
