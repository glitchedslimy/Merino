use crate::features::space::domain::{
    errors::SpaceError, repository::SpaceRepository, space::Space,
};

/// # Create Space use case
/// Creates a space with the specified name.
/// ## Params
/// * `space_name`: Name of the space to be created.
pub async fn create_space_use_case<T: SpaceRepository>(
    repo: &T,
    space_name: &str,
) -> Result<Space, SpaceError> {
    repo.create_space(space_name).await
}
