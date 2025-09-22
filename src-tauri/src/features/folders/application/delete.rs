use crate::features::folders::domain::{errors::FolderError, repository::FolderRepository};

pub async fn delete_folder_use_case<T: FolderRepository>(
    repo: &T,
    space_name: &str,
    folder_name: &str,
    folder_path: Option<&str>,
) -> Result<String, FolderError> {
    repo.delete_folder(space_name, folder_name, folder_path)
        .await
}
