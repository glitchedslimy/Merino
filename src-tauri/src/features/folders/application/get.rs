use crate::features::folders::domain::{
    errors::FolderError, folder::Folder, repository::FolderRepository,
};

pub async fn get_folders_use_case<T: FolderRepository>(
    repo: &T,
    space_name: &str,
) -> Result<Vec<Folder>, FolderError> {
    repo.get_folders(space_name).await
}
