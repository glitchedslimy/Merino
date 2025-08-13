use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;
use serde_yaml;
use tokio_util::sync::CancellationToken;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State, Window};
use tokio::fs;
use uuid::Uuid;

use crate::ai::stream_response_to_frontend;
use crate::models::{AppState, ModelResponse};

// Import models and path helpers from sibling modules
use super::models::{
    CreateNoteRequest, CreateSpaceRequest, Note, NoteContentResponse, NoteMetadata, Space,
    UpdateNoteRequest,
    Frontmatter
};
use super::paths::{ensure_app_directories_exists, get_base_path, get_note_path, get_space_path};

// --- Helper Functions for File Operations ---

// Helper to create a new note file with YAML front matter.
async fn create_note_file(
    note_path: &std::path::Path,
    name: &str,
    content: &str,
) -> Result<(), String> {
    let metadata = NoteMetadata {
        name: name.to_string(),
    };
    let yaml_string = serde_yaml::to_string(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    let full_content = format!("---\n{}---\n{}", yaml_string, content);
    fs::write(note_path, full_content)
        .await
        .map_err(|e| format!("Failed to write note file: {}", e))?;

    Ok(())
}

// Helper to read a note file and extract metadata and content.
pub async fn read_note_file(note_path: &Path) -> Result<(NoteMetadata, String), String> {
    let file_content = fs::read_to_string(note_path)
        .await
        .map_err(|e| format!("Failed to read note file: {}", e))?;

    // Manually split the content by "---"
    let parts: Vec<&str> = file_content.splitn(3, "---").collect();

    // The frontmatter should be the second part
    if parts.len() < 3 {
        println!(
            "Error: Failed to split file into 3 parts by '---'. Parts found: {}",
            parts.len()
        );
        return Err("No valid frontmatter found".to_string());
    }

    let frontmatter_str = parts[1].trim(); // Trim whitespace from the frontmatter block
    let content = parts[2].trim_start(); // Trim leading whitespace from the content

    if frontmatter_str.is_empty() {
        println!("Error: Frontmatter block is empty after splitting.");
        return Err("Frontmatter block is empty".to_string());
    }

    let metadata: NoteMetadata = serde_yaml::from_str(frontmatter_str)
        .map_err(|e| format!("Failed to parse metadata from YAML: {}", e))?;

    Ok((metadata, content.to_string()))
}

// --- Tauri Commands ---

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

// Modified `list_notes_in_space_cmd` to read UUIDs and front matter
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
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                println!("Found file: {}", file_name);

                if let Some(id_str) = file_name.strip_suffix(".md") {
                    println!("  Filename without .md: {}", id_str);

                    match Uuid::parse_str(id_str) {
                        Ok(note_id) => match read_note_file(&path).await {
                            Ok((metadata, _)) => {
                                notes.push(Note {
                                    id: note_id,
                                    name: metadata.name.clone(), // Clone the string here
                                });
                                println!("  Successfully read note: {}", metadata.name);
                                // You can now use the original metadata.name here
                            }
                            Err(e) => {
                                println!("  Failed to read note file: {}", e);
                            }
                        },
                        Err(e) => {
                            println!("  Failed to parse as UUID: {}", e);
                        }
                    }
                } else {
                    println!("  Does not have a .md suffix, skipping.");
                }
            }
        }
    }
    Ok(notes)
}

// Modified `Notes_in_space_cmd` to use UUID for filename and front matter for name
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

    let note_id = Uuid::new_v4();
    let note_filename = format!("{}.md", note_id);
    let note_path = space_path.join(&note_filename);

    create_note_file(&note_path, &req.name, &req.content).await?;

    Ok(Note {
        id: note_id,
        name: req.name,
    })
}

// Modified `get_note_content_cmd` to use UUID and front matter
#[tauri::command]
pub async fn get_note_content_cmd(
    app_handle: AppHandle,
    space_name: String,
    note_id: String, // Accepts the UUID as a string
) -> Result<NoteContentResponse, String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_id);

    if !note_path.exists() || !note_path.is_file() {
        return Err(format!(
            "Note with ID '{}' not found in space '{}'",
            note_id, space_name
        ));
    }

    let (metadata, content) = read_note_file(&note_path).await?;

    Ok(NoteContentResponse {
        name: metadata.name,
        content,
    })
}

// Modified `update_note_content_cmd` to use UUID and front matter
#[tauri::command]
pub async fn update_note_content_cmd(
    app_handle: AppHandle,
    space_name: String,
    note_id: String,
    req: UpdateNoteRequest,
) -> Result<(), String> {
    let space_path = get_space_path(&app_handle, &space_name);
    let file_path = space_path.join(format!("{}.md", note_id));

    // Read the existing file content
    let existing_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("Failed to read existing note file: {}", e))?;

    // Manually split the content to get the frontmatter
    let mut parts = existing_content.splitn(3, "---");

    let _ = parts.next(); // Discard the empty part before the first "---"
    let frontmatter = parts
        .next()
        .ok_or_else(|| "No frontmatter found in existing file".to_string())?;

    // Construct the new file content with the old frontmatter
    let full_content_to_save = format!("---\n{}\n---\n{}", frontmatter.trim(), req.content);

    // Save the new full content
    fs::write(&file_path, full_content_to_save)
        .await
        .map_err(|e| format!("Failed to write note file: {}", e))?;

    Ok(())
}

// Modified `save_note_content` to use UUID
#[tauri::command]
pub async fn save_note_content(
    app_handle: AppHandle,
    space_name: String,
    note_id: String,
    content: Vec<u8>,
) -> Result<(), String> {
    let space_path = get_space_path(&app_handle, &space_name);
    let file_path = space_path.join(format!("{}.md", note_id));

    // Convert the incoming byte array to a String
    let new_content = String::from_utf8(content)
        .map_err(|e| format!("Invalid UTF-8 sequence in content: {}", e))?;

    // Read the existing file content
    let existing_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("Failed to read existing note file: {}", e))?;

    // Manually split the content to get the frontmatter
    let mut parts = existing_content.splitn(3, "---");

    let _ = parts.next(); // Discard the empty part before the first "---"
    let frontmatter = parts
        .next()
        .ok_or_else(|| "No frontmatter found in existing file".to_string())?;

    // Construct the new file content with the old frontmatter
    // We trim the frontmatter to remove any extra newlines, and ensure the new content starts on a new line
    let full_content_to_save = format!("---\n{}\n---\n{}", frontmatter.trim(), new_content);

    // Save the new full content
    fs::write(&file_path, full_content_to_save)
        .await
        .map_err(|e| format!("Failed to write note file: {}", e))?;

    Ok(())
}

// Modified `load_note_content` to use UUID
#[tauri::command]
pub async fn load_note_content(
    app_handle: AppHandle,
    space_name: String,
    note_id: String, // Accepts the UUID as a string
) -> Result<String, String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_id);

    if !note_path.exists() {
        return Err(format!(
            "Note with ID '{}' not found in space '{}'",
            note_id, space_name
        ));
    }

    let (_, content) = read_note_file(&note_path).await?;
    Ok(content)
}

// Modified `delete_note` to use UUID
#[tauri::command]
pub async fn delete_note(
    app_handle: AppHandle,
    space_name: String,
    note_id: String, // Accepts the UUID as a string
) -> Result<bool, String> {
    let note_path = get_note_path(&app_handle, &space_name, &note_id);

    println!("Attempting to delete note at path: {:?}", note_path);

    if !note_path.exists() {
        return Err(format!(
            "Note with ID '{}' not found in space '{}' at path: {:?}",
            note_id, space_name, note_path
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
                    "Permission denied to delete note with ID '{}' at {:?}: {}",
                    note_id, note_path, e
                ))
            } else {
                Err(format!(
                    "Failed to delete note with ID '{}' at {:?}: {}",
                    note_id, note_path, e
                ))
            }
        }
    }
}

#[tauri::command]
pub async fn rename_note(
    app_handle: AppHandle,
    space_name: String,
    note_id: String,
    new_name: String,
) -> Result<Note, String> {
    // 1. Validate the new name
    if new_name.trim().is_empty() {
        return Err("New note name cannot be empty.".to_string());
    }

    // 2. Construct the file path using the helper function
    let space_path = get_space_path(&app_handle, &space_name);
    let note_path = space_path.join(format!("{}.md", &note_id));

    // 3. Read the file content asynchronously
    println!("Note path {}", &note_path.display());
    let file_content = fs::read_to_string(&note_path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // 4. Split the content to isolate frontmatter and body
    let mut parts = file_content.splitn(3, "---");
    parts.next(); // Skip the first part before the frontmatter
    let frontmatter_str = parts.next().ok_or_else(|| "No frontmatter found in file".to_string())?;
    let body = parts.next().unwrap_or_default(); // The rest of the file is the body

    // 5. Deserialize, modify, and re-serialize the frontmatter
   let mut frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
    .map_err(|e| format!("Failed to parse frontmatter YAML: {}", e))?;

    frontmatter.name = new_name.clone();

    let new_frontmatter_str = serde_yaml::to_string(&frontmatter)
        .map_err(|e| format!("Failed to serialize frontmatter YAML: {}", e))?;

    // 6. Reconstruct the full content
    let new_file_content = format!("---\n{}---\n{}", new_frontmatter_str.trim(), body);

    // 7. Write the updated content back to the file asynchronously
    fs::write(&note_path, new_file_content)
        .await
        .map_err(|e| format!("Failed to write to file: {}", e))?;

        let note_uuid = Uuid::parse_str(&note_id)
        .map_err(|e| format!("Invalid note_id format: {}", e))?;

    // 8. Return the updated Note object
    Ok(Note {
        id: note_uuid,
        name: new_name,
    })
}

// Ollama Commands for AI implementation
#[tauri::command]
pub async fn get_ollama_models_cmd() -> Result<Vec<ModelResponse>, String> {
    let ollama = Ollama::default();
    let local_models = ollama.list_local_models().await.map_err(|e| e.to_string())?;

    let mut all_model_info = Vec::new();

    for model in local_models {
        let model_info = ollama.show_model_info(model.name.clone()).await.map_err(|e| e.to_string())?;
        let model_capabilities = model_info.capabilities;
        let model_response = ModelResponse {
            name: model.name,
            capabilities: model_capabilities
        };
        all_model_info.push(model_response);
    }
    
    Ok(all_model_info)
}

#[tauri::command]
pub fn stop_ollama_stream(state: State<'_, Arc<AppState>>) {
    // Lock the Mutex and call `cancel()` on the current token.
    let token_guard = state.cancellation_token.lock().unwrap();
    token_guard.cancel();
}

#[tauri::command]
pub async fn send_to_chat_command(
	window: Window,
	messages: Vec<ChatMessage>,
	model: String,
	use_tools: bool,
	use_thinking: bool,
	state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
	let ollama = Ollama::default();
	let mut request = ChatMessageRequest::new(model.clone(), messages);

	if use_thinking {
		request = request.think(use_thinking);
	}

	if use_tools {
		// TODO: Implement tool handling
	}

	// Create a NEW token for this specific generation task
	let new_token = CancellationToken::new();

	// 💡 Lock the Mutex to safely access and modify the cancellation token inside the Arc.
	{
		let mut cancellation_token_guard = state.cancellation_token.lock().unwrap();
		cancellation_token_guard.cancel();
		*cancellation_token_guard = new_token.clone();
	} // 💡 The Mutex lock is automatically released here when the guard goes out of scope.

	// 💡 Spawn a new task to handle both the API call and the streaming.
	// This ensures the new token is in place *before* the API call is made.
	tokio::spawn(async move {
		let res = ollama.send_chat_messages_stream(request).await;

		match res {
			Ok(stream) => {
				// Pass the token to the streaming function
				let _ = stream_response_to_frontend(window, stream, new_token).await;
			}
			Err(e) => {
				eprintln!("Ollama API error: {:?}", e);
				let _ = window.emit("ollama-chat-end", {});
			}
		}
	});

	Ok(())
}