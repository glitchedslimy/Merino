use log::LevelFilter;
use tauri::Manager;
use crate::features::space::infrastructure::filesystem_repo::FileSystemSpaceRepository;
use crate::{features::notes::infrastructure::filesystem_repository::FileSystemNoteRepository, shared::logger::logger::MerinoLogger};

// Declare modules
pub mod features;
pub mod shared;

// Implement functions from infrastructure
use features::notes::infrastructure::tauri_commands::get_notes_in_space_cmd;
use features::space::infrastructure::tauri_commands::get_spaces_cmd;
use features::notes::infrastructure::tauri_commands::create_note_in_space_cmd;

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

           // Manage both repos
           app.manage(notes_repo);
           app.manage(spaces_repo);
           Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_notes_in_space_cmd,
            get_spaces_cmd,
            create_note_in_space_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}