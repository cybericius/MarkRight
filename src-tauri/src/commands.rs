use std::path::PathBuf;

use markright_core::ast::{MdNode, serialize_ast};
use markright_core::config::AppConfig;
use markright_core::frontmatter::{Frontmatter, strip_frontmatter};
use markright_core::license::{LicenseStatus, check_license_file};
use markright_core::search::{SearchResult, search_files};
use markright_core::toc::{TocEntry, extract_toc_from_ast};
use markright_core::tree::{TreeNode, build_tree};
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

use crate::state::AppState;

/// Response for `get_document` containing the full parsed document.
#[derive(Debug, Serialize)]
pub struct DocumentResponse {
    pub ast: MdNode,
    pub toc: Vec<TocEntry>,
    pub frontmatter: Option<Frontmatter>,
}

/// Open a folder and return its file tree.
///
/// Tauri commands require owned argument types for deserialization.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn open_folder(
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<TreeNode>, String> {
    let path = PathBuf::from(&path);
    if !path.is_dir() {
        return Err(format!("Not a directory: {}", path.display()));
    }

    let tree = build_tree(&path).map_err(|e| e.to_string())?;
    *state.root_folder.lock().unwrap() = Some(path);
    Ok(tree)
}

/// Get the file tree for the currently opened folder.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_tree(state: State<'_, AppState>) -> Result<Vec<TreeNode>, String> {
    let root = state.root_folder.lock().unwrap();
    let root = root.as_ref().ok_or("No folder is open")?;
    build_tree(root).map_err(|e| e.to_string())
}

/// Parse a Markdown file and return its AST, TOC, and frontmatter.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_document(path: String) -> Result<DocumentResponse, String> {
    let path = PathBuf::from(&path);
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;

    let (frontmatter, body) = strip_frontmatter(&content);
    let ast = serialize_ast(body);
    let toc = extract_toc_from_ast(&ast);

    Ok(DocumentResponse {
        ast,
        toc,
        frontmatter,
    })
}

/// Get the TOC for a Markdown file.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_toc(path: String) -> Result<Vec<TocEntry>, String> {
    let path = PathBuf::from(&path);
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;

    let (_, body) = strip_frontmatter(&content);
    let ast = serialize_ast(body);
    Ok(extract_toc_from_ast(&ast))
}

/// Search all markdown files in the open folder for a query string.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn search(query: String, state: State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    let root = state.root_folder.lock().unwrap();
    let root = root.as_ref().ok_or("No folder is open")?;
    Ok(search_files(root, &query, 50))
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

/// Load the user's persisted configuration.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_config(app: AppHandle) -> Result<AppConfig, String> {
    let path = config_path(&app)?;
    Ok(AppConfig::load(&path))
}

/// Save the user's configuration to disk.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn save_config(config: AppConfig, app: AppHandle) -> Result<(), String> {
    let path = config_path(&app)?;
    config.save(&path)
}

/// Check the license file and return its status.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn check_license(app: AppHandle) -> Result<LicenseStatus, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(check_license_file(&dir))
}

/// Return the file path passed as a CLI argument on launch, if any.
/// Returns the file path and its parent directory (for the tree view).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_initial_file(state: State<'_, AppState>) -> Option<InitialFile> {
    let path = state.initial_file.lock().unwrap().take()?;
    let folder = path.parent().map(std::path::Path::to_path_buf).unwrap_or_default();

    Some(InitialFile {
        file_path: path.to_string_lossy().into_owned(),
        folder_path: folder.to_string_lossy().into_owned(),
    })
}

/// File path and its parent folder for initial open.
#[derive(Debug, Serialize)]
pub struct InitialFile {
    pub file_path: String,
    pub folder_path: String,
}

/// Activate a license by writing the key to the config directory and verifying it.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn activate_license(key: String, app: AppHandle) -> Result<LicenseStatus, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("license.key");
    std::fs::write(&path, key.trim()).map_err(|e| e.to_string())?;
    Ok(check_license_file(&dir))
}
