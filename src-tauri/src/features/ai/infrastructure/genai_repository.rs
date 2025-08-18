use async_trait::async_trait;
use genai::{adapter::AdapterKind, Client};
use tauri::AppHandle;

use crate::features::ai::domain::{ai::ModelResponse, repository::AIRepository};

pub struct GenAIRepository{
    app_handle: AppHandle
}

impl GenAIRepository {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

#[async_trait]
impl AIRepository for GenAIRepository {
    async fn get_ai_models(&self) -> Result<Vec<ModelResponse>, String> {
       let client = Client::builder().build();

       let models = client.all_model_names(AdapterKind::Ollama).await.map_err(|e| format!("Failed to get Ollama models: {}", e))?;

       let all_models: Vec<ModelResponse> = models.into_iter().map(|model_iden| ModelResponse {
        name: model_iden.to_string()
       }).collect();
       Ok(all_models)
    }
}