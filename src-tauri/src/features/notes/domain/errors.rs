//! # Errors
//! Defines all error that could ocurr inside the notes part of the application.
use std::{io, string::FromUtf8Error};

use thiserror::Error;

use crate::shared::errors::app_errors::AppError;

/// # NoteError Enum
/// Defines the enum with all the possible error that could ocurr during a Note
/// operation in the application.
/// ## Possible Errors
/// * `NotFound`: A note hasn't been found.
/// * `InavlidId`: A note has an invalid ID.
/// * `FrontmatterParsing`: The frontmatter parser couldn't find the frontmatter.
/// * `EmptyName`: The name of the note was empty.
#[derive(Debug, Error)]
pub enum NoteError {
    #[error("Note not found in space: {0}")]
    NotFound(String),

    #[error("Invalid ID format for note IDs: {0}")]
    InvalidId(String),

    #[error("Failed to convert to markdown: {0}")]
    MarkdownConversion(#[from] FromUtf8Error),
    
    #[error("Note name cannot be empty.")]
    EmptyName,

    #[error("Notes IO error: {0}")]
    Io(#[from] io::Error),

    #[error(transparent)]
    AppError(#[from] AppError)
}
