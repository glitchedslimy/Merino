//! # Notes Domain
//! Defines all structs needed for the notes.
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
/// # Note
/// Struct that represents the note itself.
/// ## Fields
/// * `name`: The name of the note.
/// * `id`: The id of the note (taken from the NoteFrontmatter)
pub struct Note {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// # Note Response
/// Used for geting the `Reponse` of a note back when we read the notes.
/// ## Fields
/// * `name`: The note name.
/// * `content`: The contents of the Note (in markdown format).
pub struct NoteContentResponse {
    pub name: String,
    pub content: String
}

#[derive(Debug, Serialize, Deserialize)]
/// # Note Update Request
/// Used for updating the note. (In markdown format).
/// ## Fields
/// * `content`: The _content_ to be Update on the note.
pub struct UpdateNoteRequest {
    pub content: String
}