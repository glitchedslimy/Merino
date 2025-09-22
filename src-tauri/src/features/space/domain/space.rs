use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    pub route: Option<PathBuf>,
}
