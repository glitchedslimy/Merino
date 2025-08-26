use tauri::State;

use crate::features::folders::{application::{create, delete, get, update}, domain::folder::Folder, infrastructure::filesystem_repository::FileSystemFolderRepository};

#[tauri::command]
pub async fn get_folders_in_space_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str
) -> Result<Vec<Folder>, String> {
    get::get_folders_use_case(&*repo, &space_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_folder_route_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str,
    folder_name: &str,
    old_route: Option<&str>,
    new_route: Option<&str>
) -> Result<(), String> {
    update::update_folder_route_use_case(&*repo, &space_name, &folder_name, old_route, new_route).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_folder_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str,
    folder_path: Option<&str>
) -> Result<Folder, String> {
    create::create_folder_use_case(&*repo, &space_name, folder_path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_folder_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str,
    folder_name: &str,
    folder_path: Option<&str>
) -> Result<String, String> {
    delete::delete_folder_use_case(&*repo, &space_name, &folder_name, folder_path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_folder_name_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str,
    folder_name: &str,
    new_folder_name: &str,
    folder_path: Option<&str>
) -> Result<Folder, String> {
    update::update_folder_name_use_case(&*repo, &space_name, &folder_name, &new_folder_name, folder_path).await.map_err(|e| e.to_string())
}