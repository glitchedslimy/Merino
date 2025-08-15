//! # Repository
//! Implementation of the notes in this repository.
use async_trait::async_trait;

use super::note::Note;
use super::errors::NoteError;

/// # Notes Repository 
/// It implements the `list` of methods to interact with the notes in a space.
#[async_trait]
pub trait NoteRepository {
    /// # list_notes (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`: 
    /// * `space_name`: The name of the space to list notes from
    /// ## Result
    /// It returns a `Vec` of `Note` if successful, if not returns `NoteError`
    async fn list_notes(&self, space_name: &str) -> Result<Vec<Note>, NoteError>;

    /// # create_note (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`: 
    /// * `space_name`: Name of the space where the note will be created
    /// ## Result
    /// It returns a `Vec` of `Note` if successful, if not returns `NoteError`
    async fn create_note(&self, space_name: &str, note_name: &str) -> Result<(), NoteError>;
}