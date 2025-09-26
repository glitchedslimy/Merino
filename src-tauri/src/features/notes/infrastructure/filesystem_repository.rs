//! # FileSystem Repository
//! External implementation (for decoupling) all the FileSystem interactions
//! from the app _(Notes)_.
use std::{
    collections::VecDeque,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use log::{error};
use tokio::fs::{self, read_dir, read_to_string, File};

use crate::{
    features::notes::domain::{errors::NoteError, note::Note, repository::NoteRepository},
    shared::repositories::filesystem_repository::FileSystemRepository,
};

/// # FilesystemNoteRepository
/// Implementation of the NoteRepository trait.
#[derive(Clone)]
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
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        if !space_path.exists() || !space_path.is_dir() {
            let err_msg = format!("Space '{}' not found or is not a directory", space_name);
            error!("{}", err_msg);
            return Err(NoteError::NotFound(err_msg));
        }

        let mut notes: Vec<Note> = Vec::new();
        let mut directories_to_visit: VecDeque<PathBuf> = VecDeque::new();
        directories_to_visit.push_back(space_path.clone());

        while let Some(current_dir) = directories_to_visit.pop_front() {
            let mut entries = match read_dir(&current_dir).await {
                Ok(dir) => dir,
                Err(e) => {
                    error!(
                        "Failed to read directory '{}': {}",
                        current_dir.display(),
                        e
                    );
                    return Err(NoteError::NotFound(format!(
                        "Failed to read directory: {}",
                        e
                    )));
                }
            };

            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if let Some(note_name) = file_name.strip_suffix(".json") {
                            // Calculate the relative path of the note's folder from the space root.
                            // This is the key part that needs to be correct.
                            let parent_path = path.parent().ok_or(NoteError::NotFound(
                                "Failed to get parent directory".to_string(),
                            ))?;

                            let relative_path =
                                parent_path.strip_prefix(&space_path).map_err(|e| {
                                    NoteError::NotFound(format!("Couldn't perform strip: {}", e))
                                })?;
                            let folder = if relative_path.as_os_str().is_empty() {
                                None
                            } else {
                                Some(relative_path.to_str().unwrap().to_string())
                            };
                            notes.push(Note {
                                name: note_name.to_string(),
                                content: None,
                                folder,
                            });
                        }
                    }
                } else if path.is_dir() {
                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        if !dir_name.starts_with('.') {
                            directories_to_visit.push_back(path);
                        }
                    }
                }
            }
        }
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
    async fn create_note(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let mut note_path = space_path;
        let folder_name = folder_path.map(|s| s.to_string());
        if let Some(folder) = folder_name.clone() {
            note_path.push(folder);
        }
        note_path.push(format!("{}.json", note_name));

        match File::create_new(&note_path).await {
            Ok(_) => Ok(Note {
                name: note_name.to_string(),
                content: None,
                folder: folder_name,
            }),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Err(NoteError::Io(e)),
            Err(e) => Err(NoteError::Io(e)),
        }
    }

    /// # [GET] Note Content (method)
    /// Gets the content from a note in a specified space.
    /// ## Params
    /// * `space_name`: Space to pick the note content from.
    /// * `note_name`: The name of the note to pick the content from.
    /// ## Result
    /// Returns a `Note` if successful, if not a `NoteError` is returned.
    async fn get_note_content(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        // Start with the space path
        let mut note_path = space_path.clone();

        // If a folder path is provided, append it segment by segment
        if let Some(folder) = folder_path {
            note_path.push(PathBuf::from(folder));
        }

        // Append the note's filename
        note_path.push(format!("{}.json", note_name));

        if !note_path.exists() {
            let error_message = format!(
                "Note with name '{}' not found in space '{}' at path '{}'",
                note_name,
                space_name,
                note_path.display()
            );
            error!("{}", error_message);
            return Err(NoteError::NotFound(error_message));
        }

        let file_content = read_to_string(&note_path)
            .await
            .map_err(|e| NoteError::Io(e))?;

        Ok(Note {
            name: note_name.to_string(),
            content: Some(file_content.to_string()),
            folder: folder_path.map(|s| s.to_string()),
        })
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
    async fn update_note_content(
        &self,
        space_name: &str,
        note_name: &str,
        content: Vec<u8>,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let mut note_path = space_path.clone();

        if let Some(folder) = folder_path {
            note_path.push(PathBuf::from(folder));
        }
        note_path.push(format!("{}.json", note_name));

        // Convert the incoming byte array into a Markdown String
        let json_conversion =
            String::from_utf8(content).map_err(|e| NoteError::MarkdownConversion(e))?;

        let conversion = json_conversion.clone();
        fs::write(&note_path, json_conversion)
            .await
            .map_err(|e| NoteError::Io(e))?;

        // CORRECTED: Return a Note with the correct folder and content
        Ok(Note {
            name: note_name.to_string(),
            content: Some(conversion),
            folder: folder_path.map(|s| s.to_string()),
        })
    }

    /// # [DELETE] Note
    /// Deletes a note from the space specified.
    /// ## Params
    /// * `&self`
    /// * `space_name`: The space to delete the note from.
    /// * `note_name`: The name of the note to be deleted.
    /// ## Result
    /// A `String` if sucessful, A `NoteError` if not.
    async fn delete_note(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<String, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let mut note_path = space_path.clone();

        if let Some(folder) = folder_path {
            // FIX: Use PathBuf::join with Path for robust path construction.
            note_path = note_path.join(Path::new(folder));
        }

        note_path.push(format!("{}.json", note_name));

        match fs::remove_file(&note_path).await {
            Ok(_) => Ok(format!("Removed '{}' from '{}'.", note_name, space_name)),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(NoteError::NotFound(e.to_string())),
            Err(e) => Err(NoteError::Io(e)),
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
    async fn update_note_name(
        &self,
        space_name: &str,
        note_name: &str,
        new_note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError> {
        if new_note_name.trim().is_empty() {
            return Err(NoteError::EmptyName);
        }

        let space_path = self.filesystem_repo.get_space_path(space_name)?;
        let mut old_path = space_path.clone();
        let mut new_path = space_path.clone();

        if let Some(folder) = folder_path {
            old_path = old_path.join(Path::new(folder));
            new_path = new_path.join(Path::new(folder));
        }

        old_path.push(format!("{}.json", note_name));
        new_path.push(format!("{}.json", new_note_name));

        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| NoteError::Io(e))?;

        Ok(Note {
            name: new_note_name.to_string(),
            content: None,
            folder: folder_path.map(|s| s.to_string()),
        })
    }

    async fn update_note_route(
        &self,
        space_name: &str,
        note_name: &str,
        old_folder: Option<&str>,
        new_folder: Option<&str>,
    ) -> Result<Note, NoteError> {
        let space_path = self.filesystem_repo.get_space_path(space_name)?;

        if note_name.trim().is_empty() {
            return Err(NoteError::EmptyName);
        }

        // FIX: Rebuilt old_path to be more robust
        let mut old_path = space_path.clone();
        if let Some(folder) = old_folder {
            old_path = old_path.join(Path::new(folder));
        }
        old_path.push(format!("{}.json", note_name));

        // FIX: Rebuilt new_path to be more robust and validate new folder
        let mut new_path = space_path.clone();
        let new_folder_path_string = new_folder.map(|s| s.to_string());
        if let Some(folder) = new_folder {
            let folder_path = space_path.join(folder);
            if !folder_path.is_dir() {
                return Err(NoteError::NotFound(format!(
                    "Destination folder '{}' not found.",
                    folder_path.display()
                )));
            }
            new_path = new_path.join(Path::new(folder));
        }
        new_path.push(format!("{}.json", note_name));

        if old_path == new_path {
            return Err(NoteError::NotFound(
                "Source and destination paths are the same.".to_string(),
            ));
        }

        fs::rename(&old_path, &new_path)
            .await
            .map_err(|e| NoteError::Io(e))?;

        Ok(Note {
            name: note_name.to_string(),
            content: None,
            folder: new_folder_path_string,
        })
    }
}
