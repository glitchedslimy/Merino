use crate::features::notes::domain::{errors::NoteError, note::Note, repository::NoteRepository};

/// # Update Note Content
/// Saves and updates the content of a note.
/// ## Fields
/// * `repo` (&T): The repo that is implemented on the function
/// * `space_name`: The space to save / update the note to.
/// * `note_name`: The name of the note to be saved / udpated.
/// * `content`: The content to be saved / updated on the note.
#[doc(alias = "update_note_content")]
pub async fn update_note_content_use_case<T: NoteRepository>(repo: &T, space_name: &str, note_name: &str, content: Vec<u8>) -> Result<String, NoteError> {
    repo.update_note_content(space_name, note_name, content).await
}

/// # Update Note Name
/// Updates the name of a note (Rename)
/// ## Fields
/// * `repo` (&T): The repo that is implemented on the function
/// * `space_name`: The space to rename the note to.
/// * `note_name`: The name of the note to be renamed.
/// * `new_note_name`: The new note name.
#[doc(alias = "update_note_name")]
pub async fn update_note_name_use_case<T: NoteRepository>(repo: &T, space_name: &str, note_name: &str, new_note_name: &str) -> Result<Note, NoteError> {
    repo.update_note_name(space_name, note_name, new_note_name).await
}