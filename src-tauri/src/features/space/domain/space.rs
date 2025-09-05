use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    pub route: Option<PathBuf>
}