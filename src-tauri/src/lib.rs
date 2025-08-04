// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;
mod paths;
mod commands;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_spaces_cmd,
            commands::create_space_cmd,
            commands::delete_space_cmd,
            commands::list_notes_in_space_cmd,
            commands::create_note_in_space_cmd,
            commands::get_note_content_cmd,
            commands::update_note_content_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
