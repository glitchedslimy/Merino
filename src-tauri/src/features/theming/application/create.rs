use crate::features::theming::domain::repository::ThemingRepository;

pub async fn create_themes_path<T: ThemingRepository>(repo: &T) -> Result<(), String> {
    repo.create_themes_path().await
}