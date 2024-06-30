use crate::ret_code::RetCode;

#[tauri::command]
pub fn get_app_log_dir(app: tauri::AppHandle) -> Result<String, RetCode> {
    let app_log_dir = app
        .path_resolver()
        .app_log_dir()
        .ok_or(RetCode::GetAppLogPathFailed)?;
    Ok(app_log_dir.to_str().unwrap().to_string())
}
