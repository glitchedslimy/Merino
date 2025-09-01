use async_trait::async_trait;
use ollama_rs::generation::chat::ChatMessage;
use tauri::{State, Window};

use crate::{features::ai::domain::ai::ModelResponse, shared::state::state::AppState};


#[async_trait]
pub trait AIRepository {
    async fn get_ai_models(&self) -> Result<Vec<ModelResponse>, String>;
    async fn chat_with_ai(&self, window: Window, prompt: Vec<ChatMessage>, model_name: String, use_tools: bool, use_thinking: bool, state: State<'_, AppState>) -> Result<(), String>;
    async fn cancel_stream(&self, state: State<'_, AppState>) -> Result<(), String>;
}