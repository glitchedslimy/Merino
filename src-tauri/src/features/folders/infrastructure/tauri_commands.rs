use tauri::State;

use crate::features::folders::{application::get, domain::folder::Folder, infrastructure::filesystem_repository::FileSystemFolderRepository};

#[tauri::command]
pub async fn get_folders_in_space_cmd(
    repo: State<'_, FileSystemFolderRepository>,
    space_name: &str
) -> Result<Vec<Folder>, String> {
    get::get_folders_use_case(&*repo, &space_name).await.map_err(|e| e.to_string())
}