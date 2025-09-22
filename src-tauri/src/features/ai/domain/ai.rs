use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub name: String,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaWebResponse {
    pub model_name: String,
    pub description: String,
    pub sizes: Vec<String>,
    pub capabilities: Vec<String>,
    pub pulls: String,
    pub date: String,
}
