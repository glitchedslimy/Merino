use tauri::State;

use crate::features::ai::{application::get, domain::ai::ModelResponse, infrastructure::genai_repository::GenAIRepository};

#[tauri::command]
pub async fn get_ai_models_cmd(repo: State<'_, GenAIRepository>) -> Result<Vec<ModelResponse>, String> {
    get::get_ai_models_use_case(&*repo).await.map_err(|e| e.to_string())
}