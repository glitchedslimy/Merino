use async_trait::async_trait;

use crate::features::folders::domain::{errors::FolderError, folder::Folder};

#[async_trait]
pub trait FolderRepository {
    async fn get_folders(&self, space_name: &str) -> Result<Vec<Folder>, FolderError>;

    async fn update_folder_route(
        &self,
        space_name: &str,
        folder_name: &str,
        old_route: Option<&str>,
        new_route: Option<&str>,
    ) -> Result<(), FolderError>;

    async fn create_folder(
        &self,
        space_name: &str,
        folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Folder, FolderError>;

    async fn delete_folder(
        &self,
        space_name: &str,
        folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<String, FolderError>;

    async fn update_folder_name(
        &self,
        space_name: &str,
        folder_name: &str,
        new_folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Folder, FolderError>;
}
