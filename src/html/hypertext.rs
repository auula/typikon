use super::Markdown;
use serde::{Deserialize, Serialize};

// Define the HTML type
#[derive(Debug, Serialize, Deserialize)]
pub struct Hypertext {
    pub title: String,
    pub path: String,
    pub markdown: Markdown,
}

impl Hypertext {
    pub fn new(title: &str, path: &str, markdown: Markdown) -> Hypertext {
        Self {
            title: title.to_string(),
            path: path.to_string(),
            markdown,
        }
    }

    pub fn to_html(&self) -> String {
        self.markdown.to_html()
    }
}
