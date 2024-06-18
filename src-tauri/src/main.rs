// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod commands {
    pub mod manage_wt;
    pub mod os;
}

mod tools {
    pub mod fs;
}

mod config;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_config_dir = app.path_resolver().app_config_dir().unwrap();
            config::check_config_file(&app_config_dir);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::os::show_in_folder,
            commands::os::delete_folder,
            commands::os::create_folder,
            commands::manage_wt::auto_detected_wt_install_path,
            commands::manage_wt::auto_detected_wt_setting_path,
            commands::manage_wt::get_user_skins,
            commands::manage_wt::get_user_sights,
            commands::manage_wt::install_user_skin,
            commands::manage_wt::install_user_sight,
            commands::manage_wt::check_is_valid_wt_install_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
