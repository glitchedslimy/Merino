use std::{collections::VecDeque, io::ErrorKind, path::PathBuf};

use async_trait::async_trait;
use log::{error, info};
use tokio::fs::{self, read_dir};

use crate::{
    features::folders::domain::{
        errors::FolderError, folder::Folder, repository::FolderRepository,
    },
    shared::repositories::filesystem_repository::FileSystemRepository,
};

pub struct FileSystemFolderRepository {
    filesystem_repo: FileSystemRepository,
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
        let mut directories_to_visit: VecDeque<PathBuf> = VecDeque::new();
        directories_to_visit.push_back(space_path.clone());

        // Add the root folder explicitly, as its relative path is an empty string
        folders.push(Folder {
            path: Some("".to_string()),
        });

        while let Some(current_dir) = directories_to_visit.pop_front() {
            let mut entries = match read_dir(&current_dir).await {
                Ok(dir) => dir,
                Err(e) => {
                    error!(
                        "Failed to read directory '{}': {}",
                        current_dir.display(),
                        e
                    );
                    return Err(FolderError::NotFound(format!(
                        "Failed to read directory: {}",
                        e
                    )));
                }
            };

            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();

                if path.is_dir() {
                    // Get the relative path of the directory
                    let relative_path = path.strip_prefix(&space_path).map_err(|e| {
                        FolderError::NotFound(format!("Couldn't perform strip: {}", e))
                    })?;

                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        if !dir_name.starts_with('.') {
                            // Add the full relative path to the folders list
                            if let Some(path_str) = relative_path.to_str() {
                                if !path_str.is_empty() {
                                    folders.push(Folder {
                                        path: Some(path_str.to_string()),
                                    });
                                }
                            }
                            // Push the subdirectory to the queue for traversal
                            directories_to_visit.push_back(path);
                        }
                    }
                }
            }
        }

        info!("Found {} folders in space '{}'", folders.len(), space_name);
        info!("Folders: {:?}", folders);
        Ok(folders)
    }

    async fn update_folder_route(
        &self,
        space_name: &str,
        folder_name: &str,
        old_route: Option<&str>,
        new_route: Option<&str>,
    ) -> Result<(), FolderError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        let mut old_path = space_path.clone();
        if let Some(route) = old_route {
            old_path.push(route);
        }
        // Corrected: Append the folder name to the old path
        old_path.push(folder_name);

        let mut new_path = space_path.clone();
        if let Some(route) = new_route {
            new_path.push(route);
        }
        // Corrected: Append the folder name to the new path
        new_path.push(folder_name);


        // Check if the old path exists before attempting to move.
        if !old_path.exists() {
            return Err(FolderError::NotFound(format!(
                "Old folder path not found: {}",
                old_path.display()
            )));
        }

        // Attempt the rename operation.
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| FolderError::Io(e))?;

        Ok(())
    }

    async fn create_folder(
        &self,
        space_name: &str,
        folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Folder, FolderError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        let mut new_folder_path = space_path;

        if let Some(folder) = folder_path {
            new_folder_path.push(folder);
        }

        new_folder_path.push(folder_name);

        match fs::create_dir(&new_folder_path).await {
            Ok(_) => Ok(Folder {
                path: Some(new_folder_path.to_str().unwrap().to_string()),
            }),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Err(FolderError::Io(e)),
            Err(e) => Err(FolderError::Io(e)),
        }
    }

    async fn delete_folder(
        &self,
        space_name: &str,
        folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<String, FolderError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        let mut final_path = space_path.clone();

        // Append the parent path first, if it exists
        if let Some(parent_path) = folder_path {
            final_path.push(parent_path);
        }

        info!("Deleting folder: {}", final_path.display());

        match fs::remove_dir_all(&final_path).await {
            Ok(_) => Ok(format!("Removed '{}' from '{}'.", folder_name, space_name)),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(FolderError::NotFound(e.to_string())),
            Err(e) => Err(FolderError::Io(e)),
        }
    }

    async fn update_folder_name(
        &self,
        space_name: &str,
        folder_name: &str,
        new_folder_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Folder, FolderError> {
        if new_folder_name.trim().is_empty() {
            return Err(FolderError::EmptyName);
        }

        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let mut old_path = space_path.clone();
        let mut new_path = space_path.clone();

        // Append the parent folder path if it exists
        if let Some(folder) = folder_path {
            old_path.push(folder);
            new_path.push(folder);
        }

        // Append the old and new folder names to the respective paths
        old_path.push(folder_name);
        new_path.push(new_folder_name);

        info!("Rename folder, new_path: '{}' old_path: '{}'", new_path.display(), old_path.display());

        // Perform the rename operation
        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| FolderError::Io(e))?;

        // Correctly handle the folder path for the returned Folder struct
        let final_folder_path = folder_path.map(|s| s.to_string());

        Ok(Folder {
            path: final_folder_path,
        })
    }
}
