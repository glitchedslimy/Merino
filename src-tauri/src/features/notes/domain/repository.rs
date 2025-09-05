//! # Repository
//! Implementation of the notes in this repository.
use async_trait::async_trait;

use super::errors::NoteError;
use super::note::Note;

/// # Notes Repository
/// It implements the `list` of methods to interact with the notes in a space.
#[async_trait]
pub trait NoteRepository {
    /// # get_notes (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`:
    /// * `space_name`: The name of the space to list notes from
    /// ## Result
    /// It returns a `Vec` of `Note` if successful, if not returns `NoteError`
    async fn get_notes(&self, space_name: &str) -> Result<Vec<Note>, NoteError>;

    /// # create_note (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`:
    /// * `space_name`: Name of the space where the note will be created
    /// * `note_name`: The name of the note that will be created
    /// * `folder_path`: The path of the folder to create the note in.
    /// ## Result
    /// It returns a `Vec` of `Note` if successful, if not returns `NoteError`
    async fn create_note(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError>;

    /// # Get note content
    /// Gets the content of the note
    /// ## Fields
    /// * `&self`
    /// * `space_name`: Name of the space to pick the content from
    /// * `note_name`: Name of the note to pick content from
    /// * `folder_path`: The path of the folder to get the note from.
    async fn get_note_content(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError>;

    /// # Update note content
    /// Updates / Saves the content from a note
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The space of the note to be saved / updated.
    /// * `note_name`: The name of the note to be saved / updated.
    /// * `content`: A `Vec` of `u8` (bytes) that will be saved / updated.
    /// * `folder_path`: The path of the folder to update the note in.
    /// ## Result
    /// A `String` with a message saying successful creation, or a `NoteError`
    /// if not successful.
    async fn update_note_content(
        &self,
        space_name: &str,
        note_name: &str,
        content: Vec<u8>,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError>;

    /// # Delete note
    /// Deletes a note from the space specified.
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The space of the note to be deleted.
    /// * `note_name`: The name of the note to be deleted.
    /// * `folder_path`: The path of the folder to delete the note from.
    /// ## Result
    /// A `String` with a message, or a `NoteError` if not successful.
    async fn delete_note(
        &self,
        space_name: &str,
        note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<String, NoteError>;

    /// # Update note name
    /// Updates a note name from the space specified. (Rename)
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The space of the note to be deleted.
    /// * `note_name`: The name of the note to be deleted.
    /// * `new_note_name`: The new note name.
    /// * `folder_path`: The path of the folder to update the note in.
    /// ## Result
    /// A `String` with a message, or a `NoteError` if not successful.
    async fn update_note_name(
        &self,
        space_name: &str,
        note_name: &str,
        new_note_name: &str,
        folder_path: Option<&str>,
    ) -> Result<Note, NoteError>;

    /// # Update note route
    /// Updates the folder where a note is
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The space of the note to be deleted.
    /// * `note_name`: The name of the note to be deleted.
    /// * `old_folder`: The old folder where the note is.
    /// * `new_folder`: The new folder for the note.
    /// ## Result
    /// A `String` with a message, or a `NoteError` if not successful.
    async fn update_note_route(
        &self,
        space_name: &str,
        note_name: &str,
        old_folder: Option<&str>,
        new_folder: Option<&str>,
    ) -> Result<Note, NoteError>;
}
