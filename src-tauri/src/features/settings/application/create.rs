use crate::features::settings::domain::repository::SettingsRepository;


pub async fn create_settings<T: SettingsRepository>(repo: &T) -> Result<(), String> {
    repo.create_settings().await
}