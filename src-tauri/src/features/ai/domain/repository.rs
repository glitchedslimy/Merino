use async_trait::async_trait;

use crate::features::ai::domain::ai::ModelResponse;

#[async_trait]
pub trait AIRepository {
    async fn get_ai_models(&self) -> Result<Vec<ModelResponse>, String>;
}