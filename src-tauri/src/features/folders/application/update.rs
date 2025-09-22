use crate::features::folders::domain::{
    errors::FolderError, folder::Folder, repository::FolderRepository,
};

pub async fn update_folder_route_use_case<T: FolderRepository>(
    repo: &T,
    space_name: &str,
    folder_name: &str,
    old_route: Option<&str>,
    new_route: Option<&str>,
) -> Result<(), FolderError> {
    repo.update_folder_route(space_name, folder_name, old_route, new_route)
        .await
}

pub async fn update_folder_name_use_case<T: FolderRepository>(
    repo: &T,
    space_name: &str,
    folder_name: &str,
    new_folder_name: &str,
    folder_path: Option<&str>,
) -> Result<Folder, FolderError> {
    repo.update_folder_name(space_name, folder_name, new_folder_name, folder_path)
        .await
}
