use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Markdown(String);

impl Markdown {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Markdown(content.into())
    }

    pub fn to_html(&self) -> String {
        let mut html_output = String::new();
        let parser = Parser::new_ext(&self.0, Options::all());
        html::push_html(&mut html_output, parser);
        html_output
    }

    pub fn get_content(&self) -> String {
        self.0.clone()
    }
}

pub fn from_markdown(path: &Path) -> Result<Markdown> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(Markdown(content))
}
