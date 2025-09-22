use tauri::State;

use crate::features::space::{
    application::{create, delete, get},
    domain::space::Space,
    infrastructure::filesystem_repo::FileSystemSpaceRepository,
};

#[tauri::command]
pub async fn get_spaces_cmd(
    repo: State<'_, FileSystemSpaceRepository>,
) -> Result<Vec<Space>, String> {
    get::get_spaces_use_case(&*repo)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_space_cmd(
    repo: State<'_, FileSystemSpaceRepository>,
    space_name: &str,
) -> Result<Space, String> {
    create::create_space_use_case(&*repo, &space_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_space_cmd(
    repo: State<'_, FileSystemSpaceRepository>,
    space_name: &str,
) -> Result<String, String> {
    delete::delete_space_use_case(&*repo, &space_name)
        .await
        .map_err(|e| e.to_string())
}
