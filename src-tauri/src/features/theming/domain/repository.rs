use async_trait::async_trait;


#[async_trait]
pub trait ThemingRepository {
    async fn get_themes(&self) -> Result<Vec<String>, String>;
    async fn get_theme_content(&self, theme_name: String) -> Result<String, String>;
    async fn create_themes_path(&self) -> Result<(), String>;
}