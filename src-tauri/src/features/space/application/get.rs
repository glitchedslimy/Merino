use crate::features::space::domain::{
    errors::SpaceError, repository::SpaceRepository, space::Space,
};

/// # Get Spaces Use case
/// Used to list all the spaces inside the main data dir
/// ### Fields
/// * `repo` (&T): The repository that is implemented on function.
///
/// ## Returns
/// A `Result` with all the spaces in a `Vec` of `Space`, if not a `SpaceError`.
#[doc(alias = "list_spaces")]
pub async fn get_spaces_use_case<T: SpaceRepository>(repo: &T) -> Result<Vec<Space>, SpaceError> {
    repo.get_spaces().await
}
