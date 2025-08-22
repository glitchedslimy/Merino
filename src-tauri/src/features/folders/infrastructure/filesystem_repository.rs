use async_trait::async_trait;
use log::error;
use tokio::fs::read_dir;

use crate::{features::{folders::domain::{errors::FolderError, folder::Folder, repository::FolderRepository}}, shared::repositories::filesystem_repository::FileSystemRepository};

pub struct FileSystemFolderRepository {
    filesystem_repo: FileSystemRepository
}

impl FileSystemFolderRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }
}

#[async_trait]
impl FolderRepository for FileSystemFolderRepository {
    async fn get_folders(&self, space_name: &str) -> Result<Vec<Folder>, FolderError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        if !space_path.exists() || !space_path.is_dir() {
            let err_msg = format!("Space '{}' not found or is not a directory", space_name);
            error!("{}", err_msg);
            return Err(FolderError::NotFound(err_msg));
        }

        let mut folders: Vec<Folder> = Vec::new();
        let mut entries = read_dir(&space_path).await.map_err(|_| FolderError::NotFound("Failed to find space directory.".to_string()))?;

         while let Some(entry) = entries.next_entry().await.map_err(|_| FolderError::NotFound("Failed to get directory".to_string()))? {
            let path = entry.path();

            if path.is_dir() {
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !folder_name.starts_with('.') {
                        folders.push(Folder {
                            name: folder_name.to_string()
                        });
                    }
                }
            }
         }
         Ok(folders)
    }
}