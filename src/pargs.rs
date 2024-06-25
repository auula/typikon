use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Write};
use pulldown_cmark::{html, Options, Parser};

// Define the Markdown type
pub struct Markdown(String);

// Define the HTML type
pub struct Hypertext(String);

// Define the Document type
pub struct Document(String);

impl Markdown {
    // Create a new Markdown instance from a string slice
    pub fn new(content: &str) -> Markdown {
        Markdown(content.to_string())
    }

    // Convert Markdown to HTML
    pub fn to_html(&self) -> Hypertext {
        let mut html_output = String::new();
        html::push_html(&mut html_output, Parser::new_ext(&self.0, init_options()));
        Hypertext(html_output)
    }

    // Get the content of Markdown as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hypertext {
    // Create a new Hypertext instance from a Markdown instance
    pub fn new(markdown: &Markdown) -> Hypertext {
        Hypertext(markdown.0.clone())
    }

    // Get the content of Hypertext as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Document {
    // Create a new Document instance from a Hypertext instance
    pub fn new(hypertext: &Hypertext) -> Document {
        Document(hypertext.0.clone())
    }

    // Get the content of Document as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Create a file and write the Document content to it
    pub fn create_file(&self, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

// Implement the Display trait to support printing
impl fmt::Display for Markdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Hypertext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Read Markdown content from a file
pub fn from_markdown(path: &Path) -> io::Result<Markdown> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(Markdown(content))
}

// Initialize options for Markdown parsing
fn init_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options
}

// Create a document file from Hypertext content
pub fn create_document(hypertext: &Hypertext, path: &Path) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(hypertext.0.as_bytes())?;
    Ok(())
}
