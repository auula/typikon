use std::{
    fs,
    io::{self, ErrorKind},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    pub path: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub path: String,
    pub sub_chapters: Vec<SubChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubChapter {
    pub title: String,
    pub path: String,
}

impl Root {
    pub fn get_chapters(&self) -> Vec<Chapter> {
        self.chapters.clone()
    }
}

pub fn get_root() -> io::Result<Root> {
    let yaml_str = fs::read_to_string("root.yml")?;
    let book_directory = serde_yaml::from_str(&yaml_str).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("failed to parse root.yml file: {:?}", err),
        )
    })?;
    Ok(book_directory)
}
