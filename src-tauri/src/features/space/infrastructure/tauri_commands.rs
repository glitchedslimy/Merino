use tauri::State;

use crate::features::space::{application::get, domain::space::Space, infrastructure::filesystem_repo::FileSystemSpaceRepository};

#[tauri::command]
pub async fn get_spaces_cmd(repo: State<'_, FileSystemSpaceRepository>) -> Result<Vec<Space>, String> {
    get::list_spaces_use_case(&*repo).await.map_err(|e| e.to_string())
}