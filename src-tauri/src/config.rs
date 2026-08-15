use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowState {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomSkin {
    pub bg: String,
    pub card: String,
    pub accent: String,
    pub text: String,
    pub sub_text: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_key: String,
    pub interval_minutes: u64,
    pub opacity: f64,
    pub dark: bool,
    pub skin: String,
    pub custom_skin: Option<CustomSkin>,
    pub window: Option<WindowState>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            interval_minutes: 5,
            opacity: 0.92,
            dark: false,
            skin: "light".to_string(),
            custom_skin: None,
            window: None,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Config {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, s);
        }
    }
}
