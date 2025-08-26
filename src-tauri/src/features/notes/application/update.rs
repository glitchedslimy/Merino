use crate::features::notes::domain::{errors::NoteError, note::Note, repository::NoteRepository};

/// # Update Note Content
/// Saves and updates the content of a note.
/// ## Fields
/// * `repo` (&T): The repo that is implemented on the function
/// * `space_name`: The space to save / update the note to.
/// * `note_name`: The name of the note to be saved / udpated.
/// * `content`: The content to be saved / updated on the note.
/// * `folder_path`: The folder path of the note.
#[doc(alias = "update_note_content")]
pub async fn update_note_content_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
    note_name: &str,
    content: Vec<u8>,
    folder_path: Option<&str>,
) -> Result<String, NoteError> {
    repo.update_note_content(space_name, note_name, content, folder_path)
        .await
}

/// # Update Note Name
/// Updates the name of a note (Rename)
/// ## Fields
/// * `repo` (&T): The repo that is implemented on the function
/// * `space_name`: The space to rename the note to.
/// * `note_name`: The name of the note to be renamed.
/// * `new_note_name`: The new note name.
/// * `folder_path`: The folder path of the note.
#[doc(alias = "update_note_name")]
pub async fn update_note_name_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
    note_name: &str,
    new_note_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, NoteError> {
    repo.update_note_name(space_name, note_name, new_note_name, folder_path)
        .await
}

/// # Update Note Folder
/// Updates the Folder where a note is
/// ## Fields
/// * `repo` (&T): The repo that is implemented on the function
/// * `space_name`: The space to rename the note to.
/// * `note_name`: The name of the note to be renamed.
/// * `new_folder`: The new note folder.
#[doc(alias = "update_note_name")]
pub async fn update_note_route_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
    note_name: &str,
    old_folder: Option<&str>,
    new_folder: Option<&str>,
) -> Result<(), NoteError> {
    repo.update_note_route(space_name, note_name, old_folder, new_folder)
        .await
}
