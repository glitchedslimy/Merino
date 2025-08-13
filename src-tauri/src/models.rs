use std::{path::PathBuf, sync::Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_util::sync::CancellationToken;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    name: String,
    arguments: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolOutput {
    tool_call_id: String,
    output: String
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessagePayload {
    pub role: Option<String>,
    pub content: Option<String>,
    pub thinking: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ModelResponse {
   pub name: String,
    pub capabilities: Vec<String>
}

pub struct AppState {
    pub cancellation_token: Mutex<CancellationToken>
}