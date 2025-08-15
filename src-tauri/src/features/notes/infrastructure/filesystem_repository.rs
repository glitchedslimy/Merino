//! # FileSystem Repository
//! External implementation (for decoupling) all the FileSystem interactions
//! from the app _(Notes)_.
use std::io::ErrorKind;

use async_trait::async_trait;
use log::{error, info};
use tokio::fs::{read_dir, File};

use crate::{
    features::notes::{domain::{
        errors::NoteError,
        note::Note,
        repository::NoteRepository,
    }},
    shared::repositories::filesystem_repository::FileSystemRepository,
};

/// # FilesystemNoteRepository
/// Implementation of the NoteRepository trait.
pub struct FileSystemNoteRepository {
    filesystem_repo: FileSystemRepository,
}

/// Implementation of the FileSystemNoteRepository
impl FileSystemNoteRepository {
    pub fn new(filesystem_repo: FileSystemRepository) -> Self {
        Self { filesystem_repo }
    }
}

#[async_trait]
/// # NoteRepository
/// Implementation for FileSystemRepository
/// Implements all the function needed for the notes.
impl NoteRepository for FileSystemNoteRepository {
    async fn list_notes(&self, space_name: &str) -> Result<Vec<Note>, NoteError> {
        info!("Started to list the notes in space.");
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        info!("Space path {}", space_path.display());
        if !space_path.exists() || !space_path.is_dir() {
            let err_msg = format!("Space '{}' not found or is not a directory", space_name);
            error!("{}", err_msg);
            return Err(NoteError::NotFound(err_msg));
        }
        
        let mut notes: Vec<Note> = Vec::new();
        let mut entries = read_dir(&space_path).await.map_err(|_| NoteError::NotFound("Failed to find space directory.".to_string()))?;
        

        while let Some(entry) = entries.next_entry().await.map_err(|_| NoteError::NotFound("Failed to get directory".to_string()))? {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };

            if let Some(note_name) = file_name.strip_suffix(".md") {
                notes.push(Note { name: note_name.to_string() });
            }
        }
        info!("Found notes in space {}", notes.len());
        Ok(notes)

    }

    async fn create_note(&self, space_name: &str, note_name: &str) -> Result<(), NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let note_path = space_path.join(format!("{}.md", note_name));

        match File::create_new(&note_path).await {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                Err(NoteError::Io(e))
            },
            Err(e) => Err(NoteError::Io(e))
        }
    }
}
