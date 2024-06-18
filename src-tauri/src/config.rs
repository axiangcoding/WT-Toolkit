use std::path::{Path, PathBuf};

use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    wt_root_path: String,
    wt_setting_path: String,
}

const SETTING_FILE: &str = "config.json";

/// check config file exists
///
/// 检查配置文件是否存在
pub fn check_config_file(base_path: &PathBuf) {
    // 检查./config/config.json是否存在，不存在则创建
    debug!("check config file at {:?}", base_path);

    if !base_path.exists() {
        debug!("config path not exists, create it");
        std::fs::create_dir_all(&base_path).expect("create config path failed");
    }

    let config_full_path = base_path.join(SETTING_FILE);
    if !config_full_path.exists() {
        debug!("config file not exists, create it");
        let default_config = json!(AppConfig {
            wt_root_path: "".to_string(),
            wt_setting_path: "".to_string(),
        });
        let default_config_str =
            serde_json::to_string_pretty(&default_config).expect("serialize default config failed");
        std::fs::write(&config_full_path, default_config_str).expect("write default config failed");
    }
    debug!("check config file done");
}

/// get config from file
///
/// 从文件中获取配置
fn get_config(base_path: &PathBuf) -> AppConfig {
    debug!("get config from file");
    check_config_file(&base_path);

    let config_full_path = base_path.join(SETTING_FILE);
    let config_str = std::fs::read_to_string(&config_full_path).expect("read config file failed");
    let config: AppConfig = serde_json::from_str(&config_str).expect("parse config failed");
    debug!("get config from file done");
    config
}
