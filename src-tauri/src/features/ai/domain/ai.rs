use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub name: String,
    pub capabilities: Option<Vec<String>>
}

