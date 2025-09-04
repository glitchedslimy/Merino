use std::path::PathBuf;

use async_trait::async_trait;
use tokio::fs::{self, read_dir};

use crate::{features::theming::domain::repository::ThemingRepository, shared::repositories::filesystem_repository::FileSystemRepository};

#[derive(Clone)]
pub struct FileSystemThemingRepository {
    filesystem_repo: FileSystemRepository
}

impl FileSystemThemingRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }

    fn get_themes_path(&self) -> Result<PathBuf, String> {
        let mut path = self.filesystem_repo.get_base_path().map_err(|e| e.to_string())?;

        path.push("../.merino/themes");
        Ok(path)
    }

    fn get_theme_content_path(&self, theme_name: &str) -> Result<PathBuf, String> {
        let mut path = self.get_themes_path()?;
        path.push(theme_name);
        path.push(format!("{}.css", theme_name));
        Ok(path)
    }
}

#[async_trait]
impl ThemingRepository for FileSystemThemingRepository {
    async fn get_themes(&self) -> Result<Vec<String>, String> {
        let themes_path = self.get_themes_path()?;
        let mut themes = Vec::new();

        if themes_path.exists() && themes_path.is_dir() {
            let mut entries = read_dir(themes_path).await.map_err(|e| format!("Failed to read themes directory: {}", e))?;

            while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Failed to read directory entry: {}", e))? {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(theme_name) = path.file_name().and_then(|n| n.to_str()) {
                        themes.push(theme_name.to_string());
                    }
                }
            }
        }
        Ok(themes)
    }

    async fn get_theme_content(&self, theme_name: String) -> Result<String, String> {
        if theme_name == "default" {
            return Ok(String::new());
        }
        let theme_path = self.get_theme_content_path(&theme_name)?;
        let content = fs::read_to_string(&theme_path).await.map_err(|e| format!("Failed to read theme file: {}", e))?;
        Ok(content)
    }

    async fn create_themes_path(&self) -> Result<(), String> {
        let theme_path = self.get_themes_path()?;
        if !theme_path.exists() {
            fs::create_dir_all(&theme_path).await.map_err(|e| format!("Couldn't create the theming directory: {}", e))?;
        }
        Ok(())
    }
}


