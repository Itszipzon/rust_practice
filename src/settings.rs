use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
}

impl Settings {
    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = toml::from_str::<Settings>(&contents) {
                return settings;
            }
        }
        Self {}
    }

    pub fn save(&self) {
        if let Ok(content) = toml::to_string_pretty(self) {
            let path = Self::config_path();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, content);
        }
    }

    fn config_path() -> PathBuf {
      
        let proj_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("my_first_rust_game");
        if !proj_dir.exists() {
            let _ = fs::create_dir_all(&proj_dir);
        }
        proj_dir.join("settings.toml")
    }
}
