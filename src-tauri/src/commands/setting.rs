use log::{debug, error};

use crate::{
    config::{self, AppConfig},
    ret_code::RetCode,
    WrappedState,
};

#[tauri::command]
pub fn get_app_config(state: tauri::State<WrappedState>) -> Result<AppConfig, RetCode> {
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
pub fn update_app_config(
    app: tauri::AppHandle,
    state: tauri::State<WrappedState>,
    key: String,
    value: String,
) -> Result<(), RetCode> {
    // 获取可变引用
    let mut state_ref = state.lock().unwrap();

    // 从状态中提取配置
    let mut config = state_ref.as_mut().unwrap().config.clone();

    // 根据键更新配置
    match key.as_str() {
        "language" => config.language = Some(value),
        _ => return Err(RetCode::InvalidSettingKey),
    }

    // 更新状态中的配置
    state_ref.as_mut().unwrap().config = config.clone();

    // 释放状态引用以允许保存
    drop(state_ref);

    // 保存应用配置
    let app_config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or(RetCode::SaveAppSettingsFailed)?;
    config::save_config(&app_config_dir, &config).map_err(|e| {
        error!("save config error: {}", e);
        RetCode::SaveAppSettingsFailed
    })?;

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
