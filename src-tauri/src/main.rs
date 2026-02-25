// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use std::path::PathBuf;

fn main() {
    // Check if a file path was passed as a CLI argument (e.g. from file manager).
    let initial_file = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .filter(|p| p.is_file());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState::with_initial_file(initial_file))
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
            commands::get_initial_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
