use tauri::State;

use crate::features::settings::{application::{create, get, update}, infrastructure::settings_repository::FileSystemSettingsRepository};

#[tauri::command]
pub async fn create_settings_cmd(repo: State<'_, FileSystemSettingsRepository>) -> Result<(), String> {
    create::create_settings(&*repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_settings_cmd(repo: State<'_, FileSystemSettingsRepository>) -> Result<String, String> {
    get::get_settings(&*repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings_cmd(repo: State<'_, FileSystemSettingsRepository>, new_setting: String) -> Result<(), String> {
    update::update_settings(&*repo, new_setting).await.map_err(|e| e.to_string())
}