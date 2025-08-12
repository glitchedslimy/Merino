use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    pub name: String,
    pub route: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSpaceRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteRequest {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteContentResponse {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    pub name: String,
    // Add other frontmatter fields as needed
}