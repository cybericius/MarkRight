use std::path::PathBuf;
use std::sync::Mutex;

/// Shared application state managed by Tauri.
pub struct AppState {
    pub root_folder: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            root_folder: Mutex::new(None),
        }
    }
}
