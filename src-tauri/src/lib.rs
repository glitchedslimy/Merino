use crate::features::ai::infrastructure::genai_repository::GenAIRepository;
use crate::features::ai::infrastructure::tauri_commands::{create_ollama_model_cmd, delete_ollama_model_cmd};
use crate::features::folders::infrastructure::filesystem_repository::FileSystemFolderRepository;
use crate::features::search::infrastructure::search_repository::TantivySearchRepository;
use crate::features::settings::infrastructure::settings_repository::FileSystemSettingsRepository;
use crate::features::settings::infrastructure::tauri_commands::{create_settings_cmd, get_settings_cmd, update_settings_cmd};
use crate::features::space::infrastructure::filesystem_repo::FileSystemSpaceRepository;
use crate::features::theming::infrastructre::tauri_commands::{create_themes_path_cmd, get_theme_content_cmd, get_themes_cmd};
use crate::features::theming::infrastructre::theming_repository::FileSystemThemingRepository;
use crate::shared::repositories::filesystem_repository::FileSystemRepository;
use crate::shared::state::state::AppState;
use crate::{
    features::notes::infrastructure::filesystem_repository::FileSystemNoteRepository,
    shared::logger::logger::MerinoLogger,
};
use log::LevelFilter;
use tauri::Manager;

// Declare modules
pub mod features;
pub mod shared;

// Implement functions from infrastructure
use features::ai::infrastructure::tauri_commands::{
    cancel_chat_stream_cmd, chat_with_ai_cmd, get_ai_models_cmd, check_ollama_status_cmd, get_web_models_cmd
};
use features::folders::infrastructure::tauri_commands::{
    create_folder_cmd, delete_folder_cmd, get_folders_in_space_cmd, update_folder_name_cmd,
    update_folder_route_cmd,
};
use features::notes::infrastructure::tauri_commands::{
    create_note_in_space_cmd, delete_note_cmd, get_note_content_cmd, get_notes_in_space_cmd,
    search_notes_cmd, update_note_content_cmd, update_note_name_cmd, update_note_route_cmd,
};
use features::space::infrastructure::tauri_commands::{
    create_space_cmd, delete_space_cmd, get_spaces_cmd,
};
/// Static setup for the logger
static LOGGER: MerinoLogger = MerinoLogger;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Logger setup
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to setup logger");

    // Tauri Setup
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let app_data_path = app_handle.path().app_data_dir().unwrap();
            

            // Create generic filesystem repo
            let filesystem_repo = FileSystemRepository::new(app_handle.clone());

            // Create specific repo implementations using the generic ones
            let notes_repo = FileSystemNoteRepository::new(filesystem_repo.clone());

            let spaces_repo = FileSystemSpaceRepository::new(filesystem_repo.clone());
            let ai_repo = GenAIRepository::new(app_handle.clone());

            let folders_repo = FileSystemFolderRepository::new(filesystem_repo.clone());

            let search_repo =
                TantivySearchRepository::new(&app_data_path.join(".merino/search_index")).unwrap();

            let settings_repo = FileSystemSettingsRepository::new(filesystem_repo.clone());

            let theming_repo = FileSystemThemingRepository::new(filesystem_repo.clone());
            
            let index_writer = search_repo.get_index_writer().expect("Failed to get IndexWriter");

            let app_state = AppState::new(
                notes_repo.clone(),
                spaces_repo.clone(),
                folders_repo.clone(),
                search_repo.clone(),
                ai_repo.clone(),
                settings_repo.clone(),
                index_writer
            );
            app.manage(app_state);

            app.manage(notes_repo);
            app.manage(spaces_repo);
            app.manage(folders_repo);
            app.manage(ai_repo);
            app.manage(search_repo);
            app.manage(settings_repo);
            app.manage(theming_repo);

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
            get_ai_models_cmd,
            get_folders_in_space_cmd,
            update_note_route_cmd,
            update_folder_route_cmd,
            create_folder_cmd,
            delete_folder_cmd,
            update_folder_name_cmd,
            chat_with_ai_cmd,
            cancel_chat_stream_cmd,
            search_notes_cmd,
            create_settings_cmd,
            get_settings_cmd,
            update_settings_cmd,
            get_themes_cmd,
            get_theme_content_cmd,
            create_themes_path_cmd,
            check_ollama_status_cmd,
            get_web_models_cmd,
            create_ollama_model_cmd,
            delete_ollama_model_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
