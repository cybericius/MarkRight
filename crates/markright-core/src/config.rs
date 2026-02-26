use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub theme: Theme,
    pub left_panel_width: u32,
    pub right_panel_width: u32,
    pub show_left_panel: bool,
    pub show_right_panel: bool,
    pub font_family_ui: String,
    pub font_size_ui: f32,
    pub font_family_content: String,
    pub font_size_content: f32,
    pub line_height_content: f32,
    pub zoom: f32,
    pub content_width: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            left_panel_width: 256,
            right_panel_width: 224,
            show_left_panel: true,
            show_right_panel: true,
            font_family_ui: "system-ui".to_owned(),
            font_size_ui: 13.0,
            font_family_content: "serif".to_owned(),
            font_size_content: 18.0,
            line_height_content: 1.6,
            zoom: 100.0,
            content_width: "default".to_owned(),
        }
    }
}

impl AppConfig {
    /// Load config from a JSON file, falling back to defaults on any error.
    pub fn load(path: &Path) -> Self {
        let Ok(data) = fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&data).unwrap_or_default()
    }

    /// Save config to a JSON file, creating parent directories if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.font_family_ui, "system-ui");
        assert!((cfg.font_size_ui - 13.0).abs() < f32::EPSILON);
        assert_eq!(cfg.font_family_content, "serif");
        assert!((cfg.font_size_content - 18.0).abs() < f32::EPSILON);
        assert!((cfg.line_height_content - 1.6).abs() < f32::EPSILON);
        assert_eq!(cfg.left_panel_width, 256);
        assert!(cfg.show_left_panel);
    }

    #[test]
    fn load_missing_file_returns_defaults() {
        let cfg = AppConfig::load(Path::new("/nonexistent/path/config.json"));
        assert_eq!(cfg.font_family_ui, "system-ui");
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");

        let mut cfg = AppConfig::default();
        cfg.font_family_content = "monospace".to_owned();
        cfg.font_size_content = 18.0;
        cfg.left_panel_width = 300;
        cfg.save(&path).unwrap();

        let loaded = AppConfig::load(&path);
        assert_eq!(loaded.font_family_content, "monospace");
        assert!((loaded.font_size_content - 18.0).abs() < f32::EPSILON);
        assert_eq!(loaded.left_panel_width, 300);
    }

    #[test]
    fn load_corrupt_file_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, "not valid json {{{").unwrap();

        let cfg = AppConfig::load(&path);
        assert_eq!(cfg.font_family_ui, "system-ui");
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("deep").join("settings.json");

        let cfg = AppConfig::default();
        cfg.save(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn load_partial_json_fills_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, r#"{"theme":"dark","font_size_content":20.0}"#).unwrap();

        let cfg = AppConfig::load(&path);
        // Provided fields are preserved
        assert!((cfg.font_size_content - 20.0).abs() < f32::EPSILON);
        // Missing fields get defaults via #[serde(default)]
        assert_eq!(cfg.font_family_ui, "system-ui");
        assert_eq!(cfg.left_panel_width, 256);
    }
}
