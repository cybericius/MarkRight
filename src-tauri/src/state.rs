use std::path::PathBuf;
use std::sync::Mutex;

/// Shared application state managed by Tauri.
pub struct AppState {
    pub root_folder: Mutex<Option<PathBuf>>,
    /// File path passed as a CLI argument on launch.
    pub initial_file: Mutex<Option<PathBuf>>,
}

impl AppState {
    pub fn with_initial_file(file: Option<PathBuf>) -> Self {
        Self {
            root_folder: Mutex::new(None),
            initial_file: Mutex::new(file),
        }
    }
}
