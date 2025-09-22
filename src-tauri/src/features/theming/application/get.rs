use crate::features::theming::domain::{repository::ThemingRepository, theme::Theme};

pub async fn get_themes<T: ThemingRepository>(repo: &T) -> Result<Vec<Theme>, String> {
    repo.get_themes().await
}

pub async fn get_theme_content<T: ThemingRepository>(
    repo: &T,
    theme_name: String,
) -> Result<String, String> {
    repo.get_theme_content(theme_name).await
}
