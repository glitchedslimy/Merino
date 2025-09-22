use crate::features::space::domain::{errors::SpaceError, repository::SpaceRepository};

/// # Delete Space
/// Deletes the space specified.
/// ## Params
/// * `space_name`: The name of the space to be deleted.
/// ## Result
/// A `String` with a message, or `SpaceError` if not succeded.
#[doc(alias = "delete_space")]
pub async fn delete_space_use_case<T: SpaceRepository>(
    repo: &T,
    space_name: &str,
) -> Result<String, SpaceError> {
    repo.delete_space(space_name).await
}
