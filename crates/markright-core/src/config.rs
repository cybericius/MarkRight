use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: Theme,
    pub left_panel_width: u32,
    pub right_panel_width: u32,
    pub show_left_panel: bool,
    pub show_right_panel: bool,
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
        }
    }
}
