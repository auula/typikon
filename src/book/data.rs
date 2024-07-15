use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
}

impl Document {
    pub fn new(id: i32, url: String, title: String, content: String) -> Self {
        Self {
            id,
            url,
            title,
            content,
        }
    }
}