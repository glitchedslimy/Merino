use ollama_rs::generation::chat::ChatMessage;
use tauri::{State, Window};

use crate::features::ai::application::{chat, create, delete, get};
use crate::features::ai::domain::ai::OllamaWebResponse;
use crate::features::ai::{
    domain::ai::ModelResponse, infrastructure::genai_repository::GenAIRepository,
};
use crate::shared::state::state::AppState;

#[tauri::command]
pub async fn get_ai_models_cmd(
    repo: State<'_, GenAIRepository>,
) -> Result<Vec<ModelResponse>, String> {
    get::get_ai_models_use_case(&*repo)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn chat_with_ai_cmd(
    repo: State<'_, GenAIRepository>,
    window: Window,
    prompt: String,
    model_name: String,
    use_tools: bool,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    // Get the capabilities of all models
    let models = get::get_ai_models_use_case(&*repo).await?;

    // Find the specific model and check for the "thinking" capability
    let mut use_thinking = false;
    if let Some(model) = models.iter().find(|m| m.name == model_name) {
        if let Some(capabilities) = &model.capabilities {
            // Correctly access the inner Vec<String>
            if capabilities.contains(&"thinking".to_string()) {
                use_thinking = true;
            }
        }
    }

    // Create a single ChatMessage to pass to the use case
    let chat_prompt = vec![ChatMessage::user(prompt)];
    chat::chat_with_ai_use_case(
        &*repo,
        window,
        chat_prompt,
        model_name,
        use_thinking,
        use_tools,
        app_state,
    )
    .await
}

#[tauri::command]
pub async fn cancel_chat_stream_cmd(
    repo: State<'_, GenAIRepository>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    chat::cancel_chat_stream_use_case(&*repo, app_state).await
}

#[tauri::command]
pub async fn check_ollama_status_cmd(repo: State<'_, GenAIRepository>) -> Result<bool, String> {
    get::check_ollama_status_use_case(&*repo).await
}

#[tauri::command]
pub async fn get_web_models_cmd(
    repo: State<'_, GenAIRepository>,
) -> Result<Vec<OllamaWebResponse>, String> {
    get::get_web_models(&*repo).await
}

#[tauri::command]
pub async fn create_ollama_model_cmd(
    repo: State<'_, GenAIRepository>,
    model_name: String,
) -> Result<(), String> {
    create::create_ollama_model_use_case(&*repo, model_name).await
}

#[tauri::command]
pub async fn delete_ollama_model_cmd(
    repo: State<'_, GenAIRepository>,
    model_name: String,
) -> Result<(), String> {
    delete::delete_ollama_model_use_case(&*repo, model_name).await
}
