use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

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
    get_root_from_file("root.yml")
}

pub fn get_root_from_file(file_path: impl AsRef<Path>) -> io::Result<Root> {
    let file_path = file_path.as_ref();
    let yaml_str = fs::read_to_string(file_path).map_err(|error| {
        io::Error::new(
            ErrorKind::NotFound,
            format!("Failed to read root file {}: {error}", file_path.display()),
        )
    })?;
    serde_yaml::from_str(&yaml_str).map_err(|error| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse root file {}: {error}", file_path.display()),
        )
    })
}
