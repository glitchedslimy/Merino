use tauri::State;

use crate::features::theming::{
    application::{create, get},
    domain::theme::Theme,
    infrastructre::theming_repository::FileSystemThemingRepository,
};

#[tauri::command]
pub async fn get_themes_cmd(
    repo: State<'_, FileSystemThemingRepository>,
) -> Result<Vec<Theme>, String> {
    get::get_themes(&*repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_theme_content_cmd(
    repo: State<'_, FileSystemThemingRepository>,
    theme_name: String,
) -> Result<String, String> {
    get::get_theme_content(&*repo, theme_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_themes_path_cmd(
    repo: State<'_, FileSystemThemingRepository>,
) -> Result<(), String> {
    create::create_themes_path(&*repo)
        .await
        .map_err(|e| e.to_string())
}
