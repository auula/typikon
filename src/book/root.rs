use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub root: InnerRoot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InnerRoot {
    pub index: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub index: String,
    pub sub_chapters: Vec<SubChapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubChapter {
    pub title: String,
    pub path: String,
}

pub fn get_root() -> anyhow::Result<Root> {
    let content = fs::read_to_string("root.yml").map_err(|_| Error::RootFileNotFound)?;
    let root: Root =
        serde_yaml::from_str(&content).map_err(|_| Error::RootFileContentNotFormatted)?;
    Ok(root)
}
