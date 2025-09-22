use crate::features::ai::domain::{
    ai::{ModelResponse, OllamaWebResponse},
    repository::AIRepository,
};

pub async fn get_ai_models_use_case<T: AIRepository>(
    repo: &T,
) -> Result<Vec<ModelResponse>, String> {
    repo.get_ai_models().await
}

pub async fn check_ollama_status_use_case<T: AIRepository>(repo: &T) -> Result<bool, String> {
    repo.check_ollama_status().await
}

pub async fn get_web_models<T: AIRepository>(repo: &T) -> Result<Vec<OllamaWebResponse>, String> {
    repo.get_web_models().await
}
