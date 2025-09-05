use crate::features::notes::domain::{errors::NoteError, note::Note, repository::NoteRepository};

/// # Create Note Use Case
///
/// Used to create a note with a specific name "Untitled n"
/// where X will be n + 1.
/// ## Fields
/// * `repo` (&T): The repository that is implemented on the function.
/// * `space_name` (&str): The name of the space for listing the notes in there.
/// * `folder_path`: The path of the folder to create the note in.
///
/// ## Returns
/// A `Result` containing a `Vec` of `Note` if successful, or a `NoteError` if
/// a problem occurred.
#[doc(alias = "create_note")]
pub async fn create_note_use_case<T: NoteRepository>(
    repo: &T,
    space_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, NoteError> {
    let mut note_number = 1;
    // Used an "Optimistic Creation" to avoid having to scan the whole space for
    // all the notes inside it, handles it in memory.
    loop {
        let note_name = format!("Untitled {}", note_number);
        match repo.create_note(space_name, &note_name, folder_path).await {
            Ok(_) => {
                return Ok(Note {
                    name: note_name,
                    content: None,
                    folder: folder_path.map(|s| s.to_string()),
                })
            }
            Err(e) if matches!(e, NoteError::Io(_)) => {
                note_number += 1;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
