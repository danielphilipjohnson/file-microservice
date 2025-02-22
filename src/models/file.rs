use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub size: u64,
}

impl FileMetadata {
    pub fn new(name: String, mime_type: String, size: u64) -> Self {
        Self {
            name,
            mime_type,
            size,
        }
    }
}
