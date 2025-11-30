use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub last_download_directory: Option<String>,
    pub last_metadata_folder: Option<String>,
}

impl AppSettings {
    pub fn config_path() -> Result<PathBuf> {
        let project_dirs = directories::ProjectDirs::from("com", "music-tools", "music-tools")
            .context("Failed to get project directories")?;
        let config_dir = project_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;
        Ok(config_dir.join("settings.json"))
    }

    pub fn load() -> Self {
        match Self::config_path() {
            Ok(path) => {
                if path.exists() {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => match serde_json::from_str::<AppSettings>(&content) {
                            Ok(settings) => settings,
                            Err(_) => Self::default(),
                        },
                        Err(_) => Self::default(),
                    }
                } else {
                    Self::default()
                }
            }
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self).context("Failed to serialize settings")?;
        std::fs::write(&path, content).context("Failed to write settings file")?;
        Ok(())
    }

    pub fn get_download_directory(&self) -> Option<PathBuf> {
        self.last_download_directory
            .as_ref()
            .map(|s| PathBuf::from(s))
            .filter(|p| p.exists())
    }

    pub fn get_metadata_folder(&self) -> Option<PathBuf> {
        self.last_metadata_folder
            .as_ref()
            .map(|s| PathBuf::from(s))
            .filter(|p| p.exists())
    }

    pub fn set_download_directory(&mut self, path: Option<&Path>) {
        self.last_download_directory = path.map(|p| p.to_string_lossy().to_string());
    }

    pub fn set_metadata_folder(&mut self, path: Option<&Path>) {
        self.last_metadata_folder = path.map(|p| p.to_string_lossy().to_string());
    }
}
