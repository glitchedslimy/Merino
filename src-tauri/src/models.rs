use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    pub name: String,
    pub route: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSpaceRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub name: String,
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
