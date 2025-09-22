use std::path::PathBuf;

use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

use crate::{
    features::settings::domain::repository::SettingsRepository,
    shared::repositories::filesystem_repository::FileSystemRepository,
};

#[derive(Clone)]
pub struct FileSystemSettingsRepository {
    filesystem_repo: FileSystemRepository,
}

impl FileSystemSettingsRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }

    fn get_settings_path(&self) -> Result<PathBuf, String> {
        let mut path = self
            .filesystem_repo
            .get_base_path()
            .map_err(|e| e.to_string())?;
        path.push("../.merino/settings.json");
        Ok(path)
    }
}

#[async_trait]
impl SettingsRepository for FileSystemSettingsRepository {
    async fn create_settings(&self) -> Result<(), String> {
        let settings_path = self.get_settings_path()?;
        if !settings_path.exists() {
            let default_settings = "{\"primaryColor\": \"#D1600A\"}".to_string();
            fs::write(&settings_path, &default_settings)
                .await
                .map_err(|e| format!("Failed to create settings file: {}", e))?;
        }
        Ok(())
    }

    async fn get_settings(&self) -> Result<String, String> {
        let settings_path = self.get_settings_path()?;

        if settings_path.exists() {
            // Read the file asynchronously into a byte vector.
            let content_bytes = fs::read(&settings_path)
                .await
                .map_err(|e| format!("Failed to read settings: {}", e))?;

            // Convert the byte vector to a UTF-8 string.
            let content = String::from_utf8(content_bytes)
                .map_err(|e| format!("Invalid UTF-8 in settings file: {}", e))?;

            Ok(content)
        } else {
            // Return an empty string if the file does not exist.
            Ok("".to_string())
        }
    }

    async fn update_settings(&self, new_setting: String) -> Result<(), String> {
        let settings_path = self.get_settings_path()?;

        let current_settings_str = self.get_settings().await?;

        let mut current_settings: Value =
            serde_json::from_str(&current_settings_str).unwrap_or_else(|_| json!({}));

        let new_setting_value: Value = serde_json::from_str(&new_setting)
            .map_err(|e| format!("Failed to parse new setting JSON: {}", e))?;

        if let Some(map) = current_settings.as_object_mut() {
            if let Some(new_map) = new_setting_value.as_object() {
                for (key, value) in new_map {
                    map.insert(key.clone(), value.clone());
                }
            }
        }

        let merged_settings_str = serde_json::to_string(&current_settings)
            .map_err(|e| format!("Failed to serialize merged settings: {}", e))?;

        fs::write(&settings_path, &merged_settings_str)
            .await
            .map_err(|e| format!("Failed to write new settings: {}", e))?;

        Ok(())
    }
}
