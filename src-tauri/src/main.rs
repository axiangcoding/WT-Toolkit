// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
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
