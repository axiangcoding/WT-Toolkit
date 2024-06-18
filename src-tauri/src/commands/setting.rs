use log::{debug, error};

use crate::{
    config::{self, AppConfig},
    ret_code::RetCode,
    WrappedState,
};

#[tauri::command]
pub fn get_app_config(
    state: tauri::State<WrappedState>,
) -> Result<AppConfig, RetCode> {
    Ok(state.lock().unwrap().as_ref().unwrap().config.clone())
}

#[tauri::command]
pub fn save_app_config(
    app: tauri::AppHandle,
    state: tauri::State<WrappedState>,
    config: AppConfig,
) -> Result<(), RetCode> {
    let app_config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or(RetCode::SaveAppSettingsFailed)?;
    debug!("save config to {:?}", app_config_dir);
    config::save_config(&app_config_dir, &config).map_err(|e| {
        error!("save config error: {}", e);
        RetCode::SaveAppSettingsFailed
    })?;
    debug!("save config to state");
    state.lock().unwrap().as_mut().unwrap().config = config;

    Ok(())
}

#[tauri::command]
pub fn get_app_config_dir(app: tauri::AppHandle) -> Result<String, RetCode> {
    let app_config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or(RetCode::GetAppSettingsFailed)?;
    Ok(app_config_dir.to_str().unwrap().to_string())
}
