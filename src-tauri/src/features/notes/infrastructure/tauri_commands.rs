//! # Tauri Commands [Notes]
//! Implementation of the commands for the notes to expose them on tauri
//! for having them in the frontend application.
use crate::features::notes::{
    application::{create, delete, get, update},
    domain::{errors::NoteError, note::Note},
    infrastructure::filesystem_repository::FileSystemNoteRepository,
};
use tauri::State;

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
    space_name: &str,
) -> Result<Vec<Note>, String> {
    get::get_notes_use_case(&*repo, &space_name)
        .await
        .map_err(|e: NoteError| e.to_string())
}

#[tauri::command]
/// # [CREATE] note_in_space_cmd
/// Creates a note in a space.
/// ## Params
/// * `repo`: A State of the FileSystemNoteRepository
/// * `space_name`: The space of the note
/// * `folder_path`: The path of the folder to create the note in.
/// ## Result
/// Returns `Note` if succeded, if not a `String` which is a `NoteError`
pub async fn create_note_in_space_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    create::create_note_use_case(&*repo, &space_name, folder_path)
        .await
        .map_err(|e| e.to_string())
}

/// # [GET] Note Content
/// Gets the content of a note
/// ## Params
/// * `repo`: A State of the FileSystemNoteRepository
/// * `space_name`: The name of the space to pick the content from
/// * `note_name`: The name of the note inside the space to pick the note from
/// * `folder_path`: The path of the folder to get the note from.
/// ## Result
/// A `Note` if succeded, a `String` which is a `NoteError` if not.
#[tauri::command]
pub async fn get_note_content_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    note_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    get::get_note_content_use_case(&*repo, &space_name, &note_name, folder_path)
        .await
        .map_err(|e| e.to_string())
}

/// # [UPDATE] Note Content
/// Saves / Updated the content of a note.
/// ## Params
/// * `space_name`: The name of the space where the note will be updated.
/// * `note_name`: The name of the note to be updated.
/// * `content`: A `Vec` of `u8` (bytes) to update the content.
/// * `folder_path`: The folder path of the note.
#[tauri::command]
pub async fn update_note_content_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    note_name: &str,
    content: Vec<u8>,
    folder_path: Option<&str>,
) -> Result<String, String> {
    update::update_note_content_use_case(&*repo, &space_name, &note_name, content, folder_path)
        .await
        .map_err(|e| e.to_string())
}

/// # [DELETE] Note
/// Deletes a note from a space
/// ## Params
/// * `space_name`: The name of the space where the note is deleted.
/// * `note_name`: The name of the note to be deleted.
/// * `folder_path`: The path of the folder to delete the note from.
#[tauri::command]
pub async fn delete_note_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    note_name: &str,
    folder_path: Option<&str>,
) -> Result<String, String> {
    delete::delete_note_use_case(&*repo, &space_name, &note_name, folder_path)
        .await
        .map_err(|e| e.to_string())
}

/// # [UPDATE] Note Name
/// Renames a note.
/// ## Params
/// * `space_name`: The name of the space where the note will be updated.
/// * `note_name`: The name of the note to be updated.
/// * `new_note_name`: The new name of the note.
/// * `folder_path`: The folder path of the note.
#[tauri::command]
pub async fn update_note_name_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    note_name: &str,
    new_note_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    update::update_note_name_use_case(&*repo, &space_name, &note_name, &new_note_name, folder_path)
        .await
        .map_err(|e| e.to_string())
}

/// # [UPDATE] Note Route
/// Moves a file to another folder.
/// ## Params
/// * `space_name`: The name of the space where the note will be updated.
/// * `note_name`: The name of the note to be updated.
/// * `old_folder`: The old folder where the note is.
/// * `new_folder`: The new name of the note.
#[tauri::command]
pub async fn update_note_route_cmd(
    repo: State<'_, FileSystemNoteRepository>,
    space_name: &str,
    note_name: &str,
    old_folder: Option<&str>,
    new_folder: Option<&str>,
) -> Result<(), String> {
    update::update_note_route_use_case(&*repo, &space_name, &note_name, old_folder, new_folder)
        .await
        .map_err(|e| e.to_string())
}
