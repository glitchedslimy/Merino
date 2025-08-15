//! # Repository
//! Implementation of the notes in this repository.
use async_trait::async_trait;

use super::space::Space;
use super::errors::SpaceError;

/// # Notes Repository 
/// It implements the `list` of methods to interact with the notes in a space.
#[async_trait]
pub trait SpaceRepository {
    /// # list_notes (Method)
    /// Implements the method to list the notes in the space
    /// ## Fields
    /// * `&self`: 
    /// ## Result
    /// It returns a `Vec` of `Space` if successful, if not returns `SpaceError`
    async fn list_spaces(&self) -> Result<Vec<Space>, SpaceError>;
}