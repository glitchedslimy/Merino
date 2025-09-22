use crate::features::settings::domain::repository::SettingsRepository;

pub async fn get_settings<T: SettingsRepository>(repo: &T) -> Result<String, String> {
    repo.get_settings().await
}
