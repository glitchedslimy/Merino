use log::LevelFilter;
use tauri::Manager;
use crate::features::ai::infrastructure::genai_repository::GenAIRepository;
use crate::features::space::infrastructure::filesystem_repo::FileSystemSpaceRepository;
use crate::{features::notes::infrastructure::filesystem_repository::FileSystemNoteRepository, shared::logger::logger::MerinoLogger};

// Declare modules
pub mod features;
pub mod shared;

// Implement functions from infrastructure
use features::notes::infrastructure::tauri_commands::{get_notes_in_space_cmd, create_note_in_space_cmd, get_note_content_cmd, update_note_content_cmd, delete_note_cmd, update_note_name_cmd};
use features::space::infrastructure::tauri_commands::{get_spaces_cmd, create_space_cmd, delete_space_cmd};
use features::ai::infrastructure::tauri_commands::get_ai_models_cmd;
/// Static setup for the logger
static LOGGER: MerinoLogger = MerinoLogger;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Logger setup
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).expect("Failed to setup logger");

    // Tauri Setup
    tauri::Builder::default()
        .setup(|app| {
           let app_handle = app.handle();

           // Create generic filesystem repo
           let filesystem_repo = shared::repositories::filesystem_repository::FileSystemRepository::new(app_handle.clone());
           // Create specific repo implementations using the generic ones
           let notes_repo = FileSystemNoteRepository::new(filesystem_repo.clone());
           let spaces_repo = FileSystemSpaceRepository::new(filesystem_repo.clone());
           let ai_repo = GenAIRepository::new(app_handle.clone());
           // Manage both repos
           app.manage(notes_repo);
           app.manage(spaces_repo);
           app.manage(ai_repo);
           Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_notes_in_space_cmd,
            get_spaces_cmd,
            create_note_in_space_cmd,
            get_note_content_cmd,
            update_note_content_cmd,
            create_space_cmd,
            delete_space_cmd,
            delete_note_cmd,
            update_note_name_cmd,
            get_ai_models_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}