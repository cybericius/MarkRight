// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::open_folder,
            commands::get_tree,
            commands::get_document,
            commands::get_toc,
            commands::search,
            commands::get_config,
            commands::save_config,
            commands::check_license,
            commands::activate_license,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
