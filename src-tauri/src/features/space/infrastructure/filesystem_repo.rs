use async_trait::async_trait;
use log::info;
use tokio::fs::read_dir;

use crate::{features::space::{domain::{errors::SpaceError, repository::SpaceRepository, space::Space}}, shared::repositories::filesystem_repository::FileSystemRepository};


/// # FilesystemSpaceRepository
/// Implementation of the SpaceRepository trait.
pub struct FileSystemSpaceRepository {
    filesystem_repo: FileSystemRepository,
}

impl FileSystemSpaceRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }
}

#[async_trait]
impl SpaceRepository for FileSystemSpaceRepository {
    async fn list_spaces(&self) -> Result<Vec<Space>, SpaceError> {
        info!("Listing spaces in route.");
        let base_path = self.filesystem_repo.get_base_path()?;
        self.filesystem_repo.ensure_directory_exists(&base_path).await?;

        let mut spaces = Vec::new();
        let mut entries = read_dir(&base_path).await.map_err(|e| SpaceError::Io(e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| SpaceError::Io(e))? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    spaces.push(Space {
                        name: name.to_string(),
                        route: Some(path)
                    })
                }
            }
        }

        Ok(spaces)
    }
}