//! # Errors
//! Defines all error that could ocurr inside the space part of the application.
use std::io;

use thiserror::Error;

use crate::shared::errors::app_errors::AppError;

/// # SpaceError Enum
/// Defines the enum with all the possible error that could ocurr during a Space
/// operation in the application.
/// ## Possible Errors
/// * `NotFound`: A note hasn't been found.
/// * `EmptyName`: The name of the note was empty.
#[derive(Debug, Error)]
pub enum SpaceError {
    #[error("Note not found in space: {0}")]
    NotFound(String),

    #[error("Note name cannot be empty.")]
    EmptyName,

    #[error("Space IO error: {0}")]
    Io(#[from] io::Error),

    #[error(transparent)]
    AppError(#[from] AppError)
}
