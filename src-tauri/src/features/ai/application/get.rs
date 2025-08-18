use crate::features::ai::domain::{ai::ModelResponse, repository::AIRepository};

pub async fn get_ai_models_use_case<T: AIRepository>(repo: &T) -> Result<Vec<ModelResponse>, String> {
    repo.get_ai_models().await
}