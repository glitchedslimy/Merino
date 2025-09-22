use crate::features::ai::domain::repository::AIRepository;

pub async fn delete_ollama_model_use_case<T: AIRepository>(
    repo: &T,
    model_name: String,
) -> Result<(), String> {
    repo.delete_ollama_model(model_name).await
}
