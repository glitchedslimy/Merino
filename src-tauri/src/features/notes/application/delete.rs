use crate::features::notes::domain::{errors::NoteError, repository::NoteRepository};

/// # Delete Note Use Case
/// 
/// Removes a note from the space
/// ## Fields
/// * `repo` (&T): The repository that is implemented on the function.
/// * `space_name` (&str): The name of the space for deleting the note in there.
/// * `note_name`: The name of the note to be deleted.
/// 
/// ## Returns
/// A `String` if successful, or a `NoteError` if a problem ocurred.
#[doc(alias = "delete_note")]
pub async fn delete_note_use_case<T: NoteRepository>(repo: &T, space_name: &str, note_name: &str) -> Result<String, NoteError> {
    repo.delete_note(space_name, note_name).await
}