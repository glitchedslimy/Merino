use std::path::Path;

use tokio::fs;

pub async fn write_atomic(path: &Path, data: &str) -> Result<(), String> {
    let tmp_path = path.with_extension("tmp");

    fs::write(&tmp_path, data).await.map_err(|e| format!("Failed to write tmp settings file: {}", e))?;

    fs::rename(&tmp_path, path).await.map_err(|e| format!("Failed to rename tmp settings file: {}", e))?;

    Ok(())
}