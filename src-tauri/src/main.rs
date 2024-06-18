// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, sync::Mutex};

use config::AppConfig;
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tauri::Manager;

mod commands {
    pub mod log;
    pub mod os;
    pub mod setting;
    pub mod war_thunder;
}

mod tools {
    pub mod fs;
}

mod config;
mod ret_code;

type WrappedState = Mutex<Option<MyState>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyState {
    config: AppConfig,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_config_dir = app.path_resolver().app_config_dir().unwrap();
            let app_log_dir = app.path_resolver().app_log_dir().unwrap();
            let log_file = app_log_dir.join("app.log");

            if !app_log_dir.exists() {
                std::fs::create_dir_all(&app_log_dir).unwrap();
            }

            CombinedLogger::init(vec![
                TermLogger::new(
                    LevelFilter::Debug,
                    Config::default(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    File::create(log_file).unwrap(),
                ),
            ])
            .unwrap();

            config::check_and_create_config_file(&app_config_dir).unwrap();

            app.manage(Mutex::new(Some(MyState {
                config: config::get_config(&app_config_dir).unwrap(),
            })));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::os::show_in_folder,
            commands::os::delete_folder,
            commands::setting::get_app_config,
            commands::setting::save_app_config,
            commands::setting::get_app_config_dir,
            commands::log::get_app_log_dir,
            commands::war_thunder::auto_detected_wt_root_path,
            commands::war_thunder::auto_detected_wt_setting_path,
            commands::war_thunder::install_user_skin,
            commands::war_thunder::install_user_sight,
            commands::war_thunder::get_user_skins,
            commands::war_thunder::get_user_sights,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("Tauri application started successfully")
}
