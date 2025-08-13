use std::sync::{Arc, Mutex};

use tokio_util::sync::CancellationToken;

use crate::models::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod models;
mod paths;
mod ai;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(AppState {
        cancellation_token: Mutex::new(CancellationToken::new())
    });

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_spaces_cmd,
            commands::create_space_cmd,
            commands::delete_space_cmd,
            commands::list_notes_in_space_cmd,
            commands::create_note_in_space_cmd,
            commands::get_note_content_cmd,
            commands::update_note_content_cmd,
            commands::save_note_content,
            commands::load_note_content,
            commands::delete_note,
            commands::rename_note,
            commands::get_ollama_models_cmd,
            commands::send_to_chat_command,
            commands::stop_ollama_stream
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
