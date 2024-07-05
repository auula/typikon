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
    let yaml_str = fs::read_to_string("root.yml").map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("File not found root.yml file: {:?}", err),
        )
    })?;
    let book_directory = serde_yaml::from_str(&yaml_str).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse root.yml file: {:?}", err),
        )
    })?;
    Ok(book_directory)
}

pub fn get_root_from_file(file_path: &str) -> io::Result<Root> {
    let yaml_str = fs::read_to_string(file_path).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("File not found {}: {:?}", file_path, err),
        )
    })?;
    let root = serde_yaml::from_str(&yaml_str).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse {}: {:?}", file_path, err),
        )
    })?;
    Ok(root)
}