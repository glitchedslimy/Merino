use crate::features::settings::domain::repository::SettingsRepository;


pub async fn update_settings<T: SettingsRepository>(repo: &T, new_setting: String) -> Result<(), String> {
    repo.update_settings(new_setting).await
}