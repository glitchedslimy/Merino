use async_trait::async_trait;


#[async_trait]
pub trait SettingsRepository {
    async fn get_settings(&self) -> Result<String, String>;
    async fn update_settings(&self, new_setting: String) -> Result<(), String>;
    async fn create_settings(&self) -> Result<(), String>;
}