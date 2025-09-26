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

        // Read existing settings
        let current_settings_str = self.get_settings().await?;
        let mut current_settings: Value =
            serde_json::from_str(&current_settings_str).unwrap_or_else(|_| json!({}));

        // Parse the incoming new setting
        let new_setting_value: Value = serde_json::from_str(&new_setting)
            .map_err(|e| format!("Failed to parse new setting JSON: {}", e))?;

        // Merge keys safely
        match (
            current_settings.as_object_mut(),
            new_setting_value.as_object(),
        ) {
            (Some(curr_map), Some(new_map)) => {
                for (k, v) in new_map {
                    curr_map.insert(k.clone(), v.clone());
                }
            }
            _ => {
                return Err("Settings must be JSON objects".into());
            }
        }

        // Serialize merged settings once
        let merged_settings_str =
            serde_json::to_string(&current_settings).map_err(|e| e.to_string())?;

        fs::write(&settings_path, merged_settings_str)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
