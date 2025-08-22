use async_trait::async_trait;

use crate::features::folders::domain::{errors::FolderError, folder::Folder};

#[async_trait]
pub trait FolderRepository {
    async fn get_folders(&self, space_name: &str) -> Result<Vec<Folder>, FolderError>;
}
