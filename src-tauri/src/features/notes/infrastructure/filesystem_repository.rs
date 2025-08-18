//! # FileSystem Repository
//! External implementation (for decoupling) all the FileSystem interactions
//! from the app _(Notes)_.
use std::{io::ErrorKind};

use async_trait::async_trait;
use log::{error, info};
use tokio::fs::{self, read_dir, read_to_string, File};

use crate::{
    features::{notes::domain::{
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
    /// # [GET] notes (method)
    /// Gets the notes inside a space to load them in the frontend.
    /// ## Params
    /// * `space_name`: The space to pick the notes from.
    /// ## Result
    /// A `Vec` of `Note` if succeded, a `NoteError` if not.
    async fn get_notes(&self, space_name: &str) -> Result<Vec<Note>, NoteError> {
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
                notes.push(Note { name: note_name.to_string(), content: None });
            }
        }
        info!("Found notes in space {}", notes.len());
        Ok(notes)

    }

    /// # [CREATE] Note (method)
    /// Creates a note in the space specified.
    /// ## Params
    /// * `space_name`: Space where the note will be created.
    /// * `note_name`: The name of the note that will be created. _This one is
    /// handled by the backend as `Untitled n`, where n is n + 1._
    /// ## Return
    /// Creates a new note if succeded returning `()`, if not `NoteError` is
    /// returned.
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

    /// # [GET] Note Content (method)
    /// Gets the content from a note in a specified space.
    /// ## Params
    /// * `space_name`: Space to pick the note content from.
    /// * `note_name`: The name of the note to pick the content from.
    /// ## Result
    /// Returns a `Note` if successful, if not a `NoteError` is returned.
    async fn get_note_content(&self, space_name: &str, note_name: &str) -> Result<Note, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let note_path = space_path.join(format!("{}.md", note_name));

        if !note_path.exists() {
            let error_message = format!("Note with name '{}' not found in space '{}'", note_name, space_name);
            error!("{}", error_message);
            return Err(NoteError::NotFound(error_message))
        }

        let file_content = read_to_string(&note_path).await.map_err(|e| NoteError::Io(e))?;
        Ok(Note { name: note_name.to_string(), content: Some(file_content.to_string()) })
    }

    /// # [UPDATE] Note content (method)
    /// Saves or updates the note content in a space.
    /// ## Params
    /// * `space_name`: Space to where the note will be updated / saved.
    /// * `note_name`: The name of the note that will be updated / saved.
    /// * `content`: A `Vec` of `u8` (bytes) that is the content to be saved /
    /// updated.
    /// ## Result
    /// A `String` with a successful message, or a `NoteError` if not.
    async fn update_note_content(&self, space_name: &str, note_name: &str, content: Vec<u8>) -> Result<String, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let note_path = space_path.join(format!("{}.md", note_name));

        // Convert the incoming byte array into a Markdown String
        let markdown_conversion = String::from_utf8(content).map_err(|e| NoteError::MarkdownConversion(e))?;
       

        fs::write(&note_path, markdown_conversion).await.map_err(|e| NoteError::Io(e))?;
        Ok(format!("File '{}' successfully saved or updated.", note_name))
    }

    /// # [DELETE] Note
    /// Deletes a note from the space specified.
    /// ## Params
    /// * `&self`
    /// * `space_name`: The space to delete the note from.
    /// * `note_name`: The name of the note to be deleted.
    /// ## Result
    /// A `String` if sucessful, A `NoteError` if not.
    async fn delete_note(&self, space_name: &str, note_name: &str) -> Result<String, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let note_path = space_path.join(format!("{}.md", note_name));
        info!("The note path is: {}", note_path.display());
        match fs::remove_file(&note_path).await {
            Ok(_) => Ok(format!("Removed '{}' from '{}'.", note_name, space_name)),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(NoteError::NotFound(e.to_string())),
            Err(e) => Err(NoteError::Io(e))
        }
    }

    /// # [UPDATE] Note Name (Rename method)
    /// Renames the name of the note in the specified space.
    /// ## Params
    /// * `space_name`: The space to take the `note_name` from.
    /// * `note_name`: The current name of the note.
    /// * `new_note_name`: The new name for the note specified by the user.
    /// ## Result
    /// A `String` with a message, if not successfull a `NoteError`
    async fn update_note_name(&self, space_name: &str, note_name: &str, new_note_name: &str) -> Result<Note, NoteError> {
        if new_note_name.trim().is_empty() {
            return Err(NoteError::EmptyName);
        }

        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let old_path = space_path.join(format!("{}.md", note_name));
        let new_path = space_path.join(format!("{}.md", new_note_name));

        fs::rename(&old_path, &new_path).await.map_err(|e| NoteError::Io(e))?;

        Ok(Note {
            name: new_note_name.to_string(),
            content: None
        })
    }
}
