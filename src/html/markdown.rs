use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

// Define the Markdown type
#[derive(Debug, Serialize, Deserialize)]
pub struct Markdown(String);

impl Markdown {
    // Create a new Markdown instance from a string slice
    pub fn new(content: &str) -> Markdown {
        Markdown(content.to_string())
    }

    // Convert Markdown to HTML
    pub fn to_html(&self) -> String {
        let mut html_output = String::new();
        let parser = Parser::new_ext(&self.0, Options::all());
        html::push_html(&mut html_output, parser);
        html_output
    }

    // Get the content of Markdown as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn get_content(&self) -> String {
        self.0.clone()
    }
}

// Read Markdown content from a file
pub fn from_markdown(path: &Path) -> io::Result<Markdown> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(Markdown(content))
}

