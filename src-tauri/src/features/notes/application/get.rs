//! # Get **notes** module
//! This module is the one in charge of all the operations related to gets
//! such as: List and Get (Usually List will be used instead of get).

use crate::features::notes::domain::note::Note;
use crate::features::notes::domain::errors::NoteError;
use crate::features::notes::domain::repository::NoteRepository;


/// # List Notes Use Case
/// 
/// This is used to list all the notes inside a space.
/// ## Fields
/// * `repo` (&T): The repository that is implemented on the function.
/// * `space_name` (&str): The name of the space for listing the notes in there.
/// 
/// ## Returns
/// A `Result` containing a `Vec` of `Note` if successful, or a `NoteError` if
/// a problem ocurred.
#[doc(alias = "list_notes")]
pub async fn list_notes_use_case<T: NoteRepository>(repo: &T, space_name: &str) -> Result<Vec<Note>, NoteError> {
    repo.list_notes(space_name).await
}