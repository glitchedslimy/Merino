//! # Tauri Commands [Notes]
//! Implementation of the commands for the notes to expose them on tauri
//! for having them in the frontend application.
use crate::features::search::domain::search::Searchable;
use crate::{
    features::{
        notes::{
            application::{create, delete, get, update},
            domain::{errors::NoteError, note::Note},
            infrastructure::filesystem_repository::FileSystemNoteRepository,
        },
        search::{self},
    },
    shared::state::state::AppState,
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
    state: State<'_, AppState>,
    space_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    let fs_repo_lock = state.filesystem_repo.lock().await;

    let new_note = create::create_note_use_case(&*fs_repo_lock, &space_name, folder_path)
        .await
        .map_err(|e| e.to_string())?;

    let search_repo_lock = state.search_repo.lock().await;
    let mut index_writer_lock = state.index_writer.lock().await;

    search::application::index::index_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &new_note,
        space_name,
    )
    .map_err(|e| e.to_string())?;

    index_writer_lock.commit().map_err(|e| e.to_string())?;

    Ok(new_note)
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
    state: State<'_, AppState>,
    space_name: &str,
    note_name: &str,
    content: Vec<u8>,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    // 1. Acquire the filesystem lock ONCE.
    let fs_repo_lock = state.filesystem_repo.lock().await;

    // 2. Get the original note to prepare for deletion from the search index.
    let original_note =
        get::get_note_content_use_case(&*fs_repo_lock, space_name, note_name, folder_path)
            .await
            .map_err(|e| e.to_string())?;

    // 3. Update the note's content.
    let updated_note = update::update_note_content_use_case(
        &*fs_repo_lock,
        space_name,
        note_name,
        content,
        folder_path,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Now that the filesystem lock is no longer needed for the next operations,
    // it will be dropped automatically when this block ends,
    // freeing it for other threads if needed.

    // 4. Acquire search repository locks.
    let search_repo_lock = state.search_repo.lock().await;
    let mut index_writer_lock = state.index_writer.lock().await;

    // 5. Delete the old document using the original note's unique ID.
    search::application::delete::delete_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &original_note.get_unique_id(space_name),
    )
    .map_err(|e| e.to_string())?;
    
    // Commit the deletion before adding the new document
    index_writer_lock.commit().map_err(|e| e.to_string())?;

    // 6. Index the new, updated document.
    search::application::index::index_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &updated_note,
        space_name,
    )
    .map_err(|e| e.to_string())?;

    // 7. Commit all changes at once.
    index_writer_lock.commit().map_err(|e| e.to_string())?;

    Ok(updated_note)
}

/// # [DELETE] Note
/// Deletes a note from a space
/// ## Params
/// * `space_name`: The name of the space where the note is deleted.
/// * `note_name`: The name of the note to be deleted.
/// * `folder_path`: The path of the folder to delete the note from.
#[tauri::command]
pub async fn delete_note_cmd(
    state: State<'_, AppState>,
    space_name: &str,
    note_name: &str,
    folder_path: Option<&str>,
) -> Result<String, String> {
    let fs_repo_lock = state.filesystem_repo.lock().await;

    // Build the unique ID from the note's details
    let note_to_delete = Note {
        name: note_name.to_string(),
        content: None,
        folder: folder_path.map(|s| s.to_string()),
    };
    let unique_id = note_to_delete.get_unique_id(space_name);
    
    // Delete the file from the filesystem first
    let delete_note = delete::delete_note_use_case(
        &*fs_repo_lock,
        space_name,
        note_name,
        folder_path
    ).await.map_err(|e| e.to_string())?;

    let search_repo_lock = state.search_repo.lock().await;
    let mut index_writer_lock = state.index_writer.lock().await;

    // Delete the document from the index using the correct ID
    search::application::delete::delete_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &unique_id,
    ).map_err(|e| e.to_string())?;

    index_writer_lock.commit().map_err(|e| e.to_string())?;
    Ok(delete_note)
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
    state: State<'_, AppState>,
    space_name: &str,
    note_name: &str,
    new_note_name: &str,
    folder_path: Option<&str>,
) -> Result<Note, String> {
    let fs_repo_lock = state.filesystem_repo.lock().await;

    let old_note = Note {
        name: note_name.to_string(),
        content: None,
        folder: folder_path.map(|s| s.to_string()),
    };
    let old_unique_id = old_note.get_unique_id(space_name);

    let updated_note = update::update_note_name_use_case(
        &*fs_repo_lock,
        space_name,
        note_name,
        new_note_name,
        folder_path,
    ).await.map_err(|e| e.to_string())?;

    let search_repo_lock = state.search_repo.lock().await;
    let mut index_writer_lock = state.index_writer.lock().await;

    // 1. Delete the old document.
    search::application::delete::delete_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &old_unique_id,
    ).map_err(|e| e.to_string())?;
    
    // 2. Commit the deletion immediately. This is the crucial change.
    index_writer_lock.commit().map_err(|e| e.to_string())?;

    // 3. Index the new document.
    search::application::index::index_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &updated_note,
        space_name,
    ).map_err(|e| e.to_string())?;

    // 4. Commit the new index.
    index_writer_lock.commit().map_err(|e| e.to_string())?;
    
    Ok(updated_note)
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
    state: State<'_, AppState>,
    space_name: &str,
    note_name: &str,
    old_folder: Option<&str>,
    new_folder: Option<&str>,
) -> Result<Note, String> {
    let fs_repo_lock = state.filesystem_repo.lock().await;

    // Build the old unique ID from the old details
    let old_note = Note {
        name: note_name.to_string(),
        content: None,
        folder: old_folder.map(|s| s.to_string()),
    };
    let old_unique_id = old_note.get_unique_id(space_name);

    // Update the file's route on the filesystem
    let updated_note = update::update_note_route_use_case(
        &*fs_repo_lock,
        space_name,
        note_name,
        old_folder,
        new_folder,
    )
    .await
    .map_err(|e| e.to_string())?;

    let search_repo_lock = state.search_repo.lock().await;
    let mut index_writer_lock = state.index_writer.lock().await;

    // Delete the old document using the ID you just built
    search::application::delete::delete_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &old_unique_id,
    )
    .map_err(|e| e.to_string())?;

    // Commit the deletion before indexing the new document
    index_writer_lock.commit().map_err(|e| e.to_string())?;

    search::application::index::index_document_use_case(
        &*search_repo_lock,
        &mut *index_writer_lock,
        &updated_note,
        space_name,
    )
    .map_err(|e| e.to_string())?;

    index_writer_lock.commit().map_err(|e| e.to_string())?;

    Ok(updated_note)
}

#[tauri::command]
pub async fn search_notes_cmd(
    state: State<'_, AppState>,
    query: &str,
) -> Result<Vec<Note>, String> {
    let search_repo = state.search_repo.lock().await;
    let fs_repo = state.filesystem_repo.lock().await;

    let routes = search::application::search::search_documents_use_case(&*search_repo, query)
        .map_err(|e| e.to_string())?;

    let mut notes = Vec::new();
    for route in routes {
        let parts: Vec<&str> = route.split('/').collect();
        
        // This is a much more reliable way to extract the path and note name
        let space_name = parts.get(0).ok_or("Invalid space name in route")?;
        let note_name_with_ext = parts.last().ok_or("Invalid note name in route")?;
        let note_name = note_name_with_ext.trim_end_matches(".md");
        
        let folder_path_owned = if parts.len() > 2 {
            Some(parts[1..parts.len() - 1].join("/"))
        } else {
            None
        };

        if let Ok(note) = get::get_note_content_use_case(
            &*fs_repo,
            space_name,
            note_name,
            folder_path_owned.as_deref(),
        )
        .await
        {
            notes.push(note);
        }
    }
    Ok(notes)
}