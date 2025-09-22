//! # Notes Domain
//! Defines all structs needed for the notes.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// # Note
/// Struct that represents the note itself.
/// ## Fields
/// * `name`: The name of the note.
/// * `content`: The note content itself
pub struct Note {
    pub name: String,
    pub content: Option<String>,
    pub folder: Option<String>,
}
