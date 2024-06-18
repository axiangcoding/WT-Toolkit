use std::path::PathBuf;

use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub wt_root_path: String,
    pub wt_setting_path: String,
}

const SETTING_FILE: &str = "config.json";

/// check config file exists
///
/// 检查配置文件是否存在
pub fn check_config_file(base_path: &PathBuf) -> bool {
    debug!("check config file at {:?}", base_path);
    let config_full_path = base_path.join(SETTING_FILE);
    let exists = config_full_path.exists();
    debug!("config file exists: {}", exists);
    exists
}

/// check config file exists, if not, create it
///
/// 检查配置文件是否存在，如果不存在则创建
pub fn check_and_create_config_file(base_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if check_config_file(base_path) {
        return Ok(());
    }
    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    let config_full_path = base_path.join(SETTING_FILE);
    if !config_full_path.exists() {
        debug!("config file not exists, create it");
        let default_config = json!(AppConfig {
            wt_root_path: "".to_string(),
            wt_setting_path: "".to_string(),
        });
        let default_config_str = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(&config_full_path, default_config_str)?;
    }
    debug!("check config file done");
    Ok(())
}

/// get config from file
///
/// 从文件中获取配置
pub fn get_config(base_path: &PathBuf) -> Result<AppConfig, Box<dyn std::error::Error>> {
    debug!("get config from file");
    let config_full_path = base_path.join(SETTING_FILE);
    let config_str = std::fs::read_to_string(&config_full_path)?;
    let config: AppConfig = serde_json::from_str(&config_str)?;
    debug!("get config from file done");
    Ok(config)
}

/// save config to file
///
/// 保存配置到文件
pub fn save_config(
    base_path: &PathBuf,
    config: &AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("save config {:?} to file", config);
    let config_full_path = base_path.join(SETTING_FILE);
    let config_str = serde_json::to_string_pretty(&config)?;
    std::fs::write(&config_full_path, config_str)?;
    debug!("save config to file done");
    Ok(())
}
