//! # Repository
//! Implementation of the notes in this repository.
use async_trait::async_trait;

use super::space::Space;
use super::errors::SpaceError;

/// # Notes Repository 
/// It implements the `list` of methods to interact with the notes in a space.
#[async_trait]
pub trait SpaceRepository {
    /// # [GET] Spaces (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`: 
    /// ## Result
    /// It returns a `Vec` of `Space` if successful, if not returns `SpaceError`
    async fn get_spaces(&self) -> Result<Vec<Space>, SpaceError>;

    /// # [CREATE] Space (Method)
    /// Creates a with the specified name.
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The name of the space to be created.
    /// ## Result
    /// A `Space` if successful with a message, if not `SpaceError`.
    async fn create_space(&self, space_name: &str) -> Result<Space, SpaceError>;

    /// # [DELETE] Space (Method)
    /// Deletes a space with the specified name.
    /// ## Fields
    /// * `&self`
    /// * `space_name`: The name of the space to be deleted.
    /// ## Result
    /// A `String` if successful with a message, if not `SpaceError`.
    async fn delete_space(&self, space_name: &str) -> Result<String, SpaceError>;
}