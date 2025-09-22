use std::io::ErrorKind;

use async_trait::async_trait;
use log::info;
use tokio::fs::{self, read_dir};

use crate::{
    features::space::domain::{errors::SpaceError, repository::SpaceRepository, space::Space},
    shared::repositories::filesystem_repository::FileSystemRepository,
};

/// # FilesystemSpaceRepository
/// Implementation of the SpaceRepository trait.
#[derive(Clone)]
pub struct FileSystemSpaceRepository {
    filesystem_repo: FileSystemRepository,
}

impl FileSystemSpaceRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }
}

#[async_trait]
/// # Space Repository
/// _Implementation for FileSystemSpaceRepository_
///
/// Implements all the methods for treating spaces.
impl SpaceRepository for FileSystemSpaceRepository {
    /// # [GET] Spaces
    /// Gets all the space in the default route (for now)
    /// ## Fields
    /// * `&self`: Contains the repo.
    /// ## Result
    /// A `Vec` of `Space` if succeded, `SpaceError` if not.
    async fn get_spaces(&self) -> Result<Vec<Space>, SpaceError> {
        let base_path = self.filesystem_repo.get_base_path()?;
        self.filesystem_repo
            .ensure_directory_exists(&base_path)
            .await?;

        let mut spaces = Vec::new();
        let mut entries = read_dir(&base_path).await.map_err(|e| SpaceError::Io(e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| SpaceError::Io(e))? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    spaces.push(Space {
                        name: name.to_string(),
                        route: Some(path),
                    })
                }
            }
        }

        Ok(spaces)
    }

    /// # [CREATE] Space
    /// Creates a space with the specified name.
    /// ## Params
    /// * `space_name`: Name of the space to be created.
    /// ## Result
    /// A `Space` if succeded, a `SpaceError` if not.
    async fn create_space(&self, space_name: &str) -> Result<Space, SpaceError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        self.filesystem_repo
            .ensure_directory_exists(&space_path)
            .await
            .map_err(|e| SpaceError::AppError(e))?;

        Ok(Space {
            name: space_name.to_string(),
            route: Some(space_path),
        })
    }

    /// # [DELETE] Space
    /// Deletes a space with the specified name.
    /// ## Params
    /// * `space_name`: Name of the space to be deleted.
    /// ## Result
    /// A `String` if succeded, a `SpaceError` if not.
    async fn delete_space(&self, space_name: &str) -> Result<String, SpaceError> {
        let space_path = self.filesystem_repo.get_space_path(&space_name)?;

        match fs::remove_dir_all(&space_path).await {
            Ok(_) => Ok(format!("Removed '{}' space.", space_name)),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(SpaceError::NotFound(e.to_string())),
            Err(e) => Err(SpaceError::Io(e)),
        }
    }
}
