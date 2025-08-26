use crate::features::folders::domain::{errors::FolderError, folder::Folder, repository::FolderRepository};

pub async fn create_folder_use_case<T: FolderRepository>(repo: &T, space_name: &str, folder_path: Option<&str>) -> Result<Folder, FolderError> {
    let mut folder_number = 1;

    loop {
        let folder_name = format!("Untitled {}", folder_number);
        match repo.create_folder(space_name, &folder_name, folder_path).await {
            Ok(_) => {
                let new_folder_path = match folder_path {
                    Some(parent_path) => format!("{}/{}", parent_path, folder_name),
                    None => folder_name,
                };
                return Ok(Folder { path: Some(new_folder_path) })
            }
            Err(e) if matches!(e, FolderError::Io(_)) => {
                folder_number += 1;
                continue;
            }
            Err(e) => return Err(e)
        }
    }
}