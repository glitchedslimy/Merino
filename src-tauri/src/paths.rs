use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tokio::fs;

const BASE_DIR_NAME: &str = "notalia";

pub fn get_base_path(app_handle: &AppHandle) -> PathBuf {
    return app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| {
            eprintln!("Could not determine app data directory, falling back to current directory.");
            std::env::current_dir().unwrap_or_else(|_| panic!("Failed to get current directory."))
        })
        .join(BASE_DIR_NAME);
}

pub fn get_space_path(app_handle: &AppHandle, space_name: &str) -> PathBuf {
    return get_base_path(app_handle).join(space_name);
}

pub fn get_note_path(app_handle: &AppHandle, space_name: &str, note_name: &str) -> PathBuf {
    get_space_path(app_handle, space_name).join(format!("{}.md", note_name))
}

pub async fn ensure_app_directories_exists(app_handle: &AppHandle) -> Result<(), String> {
    let app_data_root_dir = app_handle
        .path()
        .app_data_dir()
        .or_else(|_| Err("Could not determine application data directory for creation"))?;

    if !app_data_root_dir.exists() {
        return fs::create_dir_all(&app_data_root_dir)
            .await
            .map_err(|e| format!("Failed to create app data root directory. {}", e));
    }

    let notes_base_path = app_data_root_dir.join(BASE_DIR_NAME);

    if !notes_base_path.exists() {
        return fs::create_dir_all(&notes_base_path)
            .await
            .map_err(|e| format!("Failed to create notes base directory. {}", e));
    }

    Ok(())
}
