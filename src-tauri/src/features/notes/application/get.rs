//! # Get **notes** module
//! This module is the one in charge of all the operations related to gets
//! such as: List and Get (Usually List will be used instead of get).

use crate::features::notes::domain::errors::NoteError;
use crate::features::notes::domain::note::Note;
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
/// a problem occurred.
#[doc(alias = "get_notes")]
pub async fn get_notes_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
) -> Result<Vec<Note>, NoteError> {
    repo.get_notes(space_name).await
}

/// # Get Note content
/// Used to Read the content of a file
///
/// ## Fields
/// * `repo`: The repository that is implemented on the function.
/// * `space_name`: The name space of the note to be readed.
/// * `note_name`: The name of the note to be readed.
#[doc(alias = "get_note_content")]
pub async fn get_note_content_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
    note_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, NoteError> {
    repo.get_note_content(space_name, note_name, folder_path)
        .await
}
