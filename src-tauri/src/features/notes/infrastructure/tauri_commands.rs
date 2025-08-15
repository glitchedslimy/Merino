//! # Tauri Commands [Notes]
//! Implementation of the commands for the notes to expose them on tauri
//! for having them in the frontend application.
use tauri::State;
use crate::features::{notes::{application::{create, get}, domain::{errors::NoteError, note::Note}, infrastructure::filesystem_repository::FileSystemNoteRepository}};


#[tauri::command]
/// # [GET] notes_in_space_cmd
/// Obtains _all_ the notes in a space.
/// ## Params
/// * `repo`: A State of the FileSystemNoteRepository
/// * `space_name`: Name of the space
/// ## Result
/// Returns a `Vec` of `Note` if succeded, if not a `String` which is a `NoteError`
pub async fn get_notes_in_space_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str
) -> Result<Vec<Note>, String> {
    get::list_notes_use_case(&*repo, &space_name).await.map_err(|e: NoteError| e.to_string())
}

#[tauri::command]
/// # [CREATE] note_in_space_cmd
/// Creates a note in a space.
/// ## Params
/// * `repo`: A State of the FileSystemNoteRepository
/// * `note_path`: Path of the note
/// ## Result
/// Returns a `Vec` of `Note` if succeded, if not a `String` which is a `NoteError`
pub async fn create_note_in_space_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str
) -> Result<Note, String> {
    create::create_note_use_case(&*repo, &space_name).await.map_err(|e| e.to_string())
}