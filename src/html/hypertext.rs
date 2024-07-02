use serde::{Deserialize, Serialize};

use super::Markdown;

// Define the HTML type
#[derive(Debug, Serialize, Deserialize)]
pub struct Hypertext {
    pub title: String,
    pub path: String,
    pub markdown: Markdown,
}

impl Hypertext {
    pub fn new(title: &str, path: &str, markdown: Markdown) -> Hypertext {
        Hypertext {
            title: title.to_string(),
            path: path.to_string(),
            markdown: markdown,
        }
    }

    pub fn to_html(&self) -> String {
        self.markdown.to_html()
    }
}
