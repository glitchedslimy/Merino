use ollama_rs::generation::chat::ChatMessage;
use tauri::{State, Window};

use crate::{features::ai::domain::repository::AIRepository, shared::state::state::AppState};

pub async fn chat_with_ai_use_case<T: AIRepository>(
    repo: &T,
    window: Window,
    prompt: Vec<ChatMessage>,
    model_name: String,
    use_thinking: bool,
    use_tools: bool,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    repo.chat_with_ai(
        window,
        prompt,
        model_name,
        use_thinking,
        use_tools,
        app_state,
    )
    .await
}

pub async fn cancel_chat_stream_use_case<T: AIRepository>(
    repo: &T,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    repo.cancel_stream(app_state).await
}
