use std::{
    io::ErrorKind,
    path::{PathBuf},
};

use log::{debug, error};
use tauri::{AppHandle, Manager};
use tokio::fs::{self};

use crate::{shared::errors::app_errors::AppError};
/// Constant for defining the BASE DIR of the app

const BASE_DIR_NAME: &str = "merino";

/// # Filesystem Repository
/// A generic implementation of all filesystem interactions.
#[derive(Clone, Debug)]
pub struct FileSystemRepository {
    app_handle: AppHandle,
}

/// Implementation for the generic repository.
impl FileSystemRepository {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
    /// _[PUBLIC]_ Get the base path of the application.
    pub fn get_base_path(&self) -> Result<PathBuf, AppError> {
        debug!("Retrieving the app base path.");
        let app_data_dir = self.app_handle.path().app_data_dir().map_err(|e| {
            error!("Failed to get the app data directory, reason: {}", e);
            AppError::TauriIo(e)
        })?;

        let base_path = app_data_dir.join(BASE_DIR_NAME);
        debug!(
            "Base path successfully resolved to: {}",
            base_path.display()
        );
        Ok(base_path)
    }

    /// _[PUBLIC]_ Get the path for a specific space.
    pub fn get_space_path(&self, space_name: &str) -> Result<PathBuf, AppError> {
        Ok(self.get_base_path()?.join(space_name))
    }

    /// _[PUBLIC]_ Ensure a directory exists, creating it if necessary.
    pub async fn ensure_directory_exists(&self, path: &PathBuf) -> Result<(), AppError> {
        match fs::create_dir_all(path).await {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(AppError::Io(e)),
        }
    }
}
