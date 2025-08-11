use tauri::AppHandle;
use tokio::fs;
use tokio::io::AsyncWriteExt; // For writing files asynchronously

// Import models and path helpers from sibling modules
use super::models::{
    CreateNoteRequest, CreateSpaceRequest, Note, NoteContentResponse, Space, UpdateNoteRequest,
};
use super::paths::{ensure_app_directories_exists, get_base_path, get_note_path, get_space_path};

#[tauri::command]
pub async fn list_spaces_cmd(app_handle: AppHandle) -> Result<Vec<Space>, String> {
    let base_path = get_base_path(&app_handle);

    if !base_path.exists() {
        if let Err(e) = fs::create_dir_all(&base_path).await {
            return Err(format!(
                "Failed to create base directory {}: {}",
                base_path.display(),
                e
            ));
        }
    }

    let mut spaces = Vec::new();
    let mut entries = match fs::read_dir(&base_path).await {
        Ok(e) => e,
        Err(e) => return Err(format!("Failed to read base directory: {}", e)),
    };

    while let Some(entry) = entries.next_entry().await.ok().flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                spaces.push(Space {
                    name: name.to_string(),
                    route: Some(path),
                });
            }
        }
    }
    println!("{:?}", spaces);
    Ok(spaces)
}

#[tauri::command]
pub async fn create_space_cmd(
    app_handle: AppHandle,
    req: CreateSpaceRequest,
) -> Result<Space, String> {
    ensure_app_directories_exists(&app_handle).await?;
    let space_path = get_space_path(&app_handle, &req.name);
    if space_path.exists() {
        return Err(format!("Space '{}' already exists", req.name));
    }
    println!("{}", &space_path.display());

    match fs::create_dir(&space_path).await {
        Ok(_) => Ok(Space {
            name: req.name,
            route: Some(space_path),
        }),
        Err(e) => Err(format!("Failed to create space '{}': {}", req.name, e)),
    }
}

#[tauri::command]
pub async fn delete_space_cmd(app_handle: AppHandle, space_name: String) -> Result<(), String> {
    let space_path = get_space_path(&app_handle, &space_name);

    if !space_path.exists() || !space_path.is_dir() {
        return Err(format!(
            "Space '{}' not found or is not a directory",
            space_name
        ));
    }

    match fs::remove_dir_all(&space_path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete space '{}': {}", space_name, e)),
    }
}

#[tauri::command]
pub async fn list_notes_in_space_cmd(
    app_handle: AppHandle,
    space_name: String,
) -> Result<Vec<Note>, String> {
    let space_path = get_space_path(&app_handle, &space_name);

    if !space_path.exists() || !space_path.is_dir() {
        return Err(format!(
            "Space '{}' not found or is not a directory",
            space_name
        ));
    }

    let mut notes = Vec::new();
    let mut entries = match fs::read_dir(&space_path).await {
        Ok(e) => e,
        Err(e) => return Err(format!("Failed to read space directory: {}", e)),
    };

    while let Some(entry) = entries.next_entry().await.ok().flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let note_name = name.trim_end_matches(".md").to_string();
                notes.push(Note { name: note_name });
            }
        }
    }
    Ok(notes)
}

#[tauri::command]
pub async fn create_note_in_space_cmd(
    app_handle: AppHandle,
    space_name: String,
    req: CreateNoteRequest,
) -> Result<Note, String> {
    let space_path = get_space_path(&app_handle, &space_name);
    if !space_path.exists() || !space_path.is_dir() {
        return Err(format!(
            "Space '{}' not found or is not a directory",
            space_name
        ));
    }

    let note_path = get_note_path(&app_handle, &space_name, &req.name);
    if note_path.exists() {
        return Err(format!(
            "Note '{}' already exists in space '{}'",
            req.name, space_name
        ));
    }

    match fs::File::create(&note_path).await {
        Ok(mut file) => match file.write_all(req.content.as_bytes()).await {
            Ok(_) => Ok(Note { name: req.name }),
            Err(e) => Err(format!(
                "Failed to write note content to {}: {}",
                note_path.display(),
                e
            )),
        },
        Err(e) => Err(format!(
            "Failed to create note file {}: {}",
            note_path.display(),
            e
        )),
    }
}

#[tauri::command]
pub async fn get_note_content_cmd(
    app_handle: AppHandle,
    space_name: String,
    note_name: String,
) -> Result<NoteContentResponse, String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_name);

    if !note_path.exists() || !note_path.is_file() {
        return Err(format!(
            "Note '{}' not found in space '{}'",
            note_name, space_name
        ));
    }

    match fs::read_to_string(&note_path).await {
        Ok(content) => Ok(NoteContentResponse {
            name: note_name,
            content,
        }),
        Err(e) => Err(format!(
            "Failed to read note content from {}: {}",
            note_path.display(),
            e
        )),
    }
}

#[tauri::command]
pub async fn update_note_content_cmd(
    app_handle: AppHandle,
    space_name: String,
    note_name: String,
    req: UpdateNoteRequest,
) -> Result<(), String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_name);

    if !note_path.exists() || !note_path.is_file() {
        return Err(format!(
            "Note '{}' not found in space '{}'",
            note_name, space_name
        ));
    }

    match fs::write(&note_path, req.content.as_bytes()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Failed to update note content for {}: {}",
            note_path.display(),
            e
        )),
    }
}

#[tauri::command]
pub async fn save_note_content(
    app_handle: AppHandle,
    space_name: String,
    note_name: String,
    content: Vec<u8>,
) -> Result<(), String> {
    let base_path = get_base_path(&app_handle);
    let space_path = base_path.join(space_name);

    if !space_path.exists() {
        return Err(format!(
            "Space directory '{}' does not exists.",
            space_path.display()
        ));
    }

    let note_path = space_path.join(format!("{}.md", note_name));

    match fs::write(&note_path, content).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Failed to save note to {} : {}",
            note_path.display(),
            e
        )),
    }
}

#[tauri::command]
pub async fn load_note_content(
    app_handle: AppHandle,
    space_name: String,
    note_name: String,
) -> Result<String, String> {
    let base_path = get_base_path(&app_handle);
    let space_path = base_path.join(space_name);

    if !space_path.exists() {
        return Err(format!(
            "Space directory '{}' does not exist.",
            space_path.display()
        ));
    }

    let note_path = space_path.join(format!("{}.md", note_name));

    match fs::read(&note_path).await {
        Ok(bytes) => {
            let content = String::from_utf8_lossy(&bytes).to_string();
            Ok(content)
        }
        Err(e) => Err(format!(
            "Failed to load note from {} : {}",
            note_path.display(),
            e
        )),
    }
}

#[tauri::command]
pub async fn delete_note(
    app_handle: AppHandle,
    space_name: String,
    note_name: String,
) -> Result<bool, String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_name);

    println!("Attempting to delete note at path: {:?}", note_path);

    if !note_path.exists() {
        return Err(format!(
            "Note '{}' not found in space '{}' at path: {:?}",
            note_name, space_name, note_path
        ));
    }

    match fs::remove_file(&note_path).await {
        Ok(_) => {
            println!("Successfully deleted note: {:?}", note_path);
            Ok(true) // Indicate success
        }
        Err(e) => {
            // Handle various file system errors
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Err(format!(
                    "Permission denied to delete note '{}' at {:?}: {}",
                    note_name, note_path, e
                ))
            } else {
                Err(format!(
                    "Failed to delete note '{}' at {:?}: {}",
                    note_name, note_path, e
                ))
            }
        }
    }
}
